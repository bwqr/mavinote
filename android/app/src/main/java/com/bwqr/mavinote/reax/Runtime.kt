package com.bwqr.mavinote.reax

import android.util.Log
import com.bwqr.mavinote.BuildConfig
import com.bwqr.mavinote.models.NoteError
import com.novi.bincode.BincodeDeserializer
import com.novi.serde.DeserializationError
import kotlinx.coroutines.CancellableContinuation
import kotlinx.coroutines.cancel
import kotlinx.coroutines.channels.ProducerScope
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.suspendCancellableCoroutine
import java.util.concurrent.Semaphore
import kotlin.concurrent.thread
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException
import kotlin.random.Random

typealias AsyncStart = (Int) -> Long
typealias SyncStart = () -> ByteArray

interface Future {
    // Return value indicates whether the future is completed or not
    fun handle(bytes: ByteArray): Boolean

    fun abort()
}

class Stream<T> constructor(
    private val scope: ProducerScope<T>,
    private val deserialize: Deserialize<T>,
) : Future {
    private var joinHandle: Long? = null

    override fun handle(bytes: ByteArray): Boolean {
        val deserializer = BincodeDeserializer(bytes)

        return when (deserializer.deserialize_variant_index()) {
            0 -> {
                when (val either = Either.Deserialize(deserialize, NoteError).deserialize(deserializer)) {
                    is Either.Success -> scope.trySend(either.value)
                    is Either.Failure -> scope.cancel("", either.value)
                }

                false
            }

            1 -> {
                scope.close()
                true
            }

            else -> throw DeserializationError("Unknown variant index for Stream")
        }
    }

    override fun abort() {
        val joinHandle = joinHandle ?: throw Error("A Stream without a joinHandle is being aborted")

        _abort(joinHandle)
    }

    fun setJoinHandle(handle: Long) {
        joinHandle = handle
    }
}

class Once<T> constructor(
    private val continuation: CancellableContinuation<T>,
    private val deserialize: Deserialize<T>,
) : Future {
    private var joinHandle: Long? = null

    override fun handle(bytes: ByteArray): Boolean {
        val deserializer = BincodeDeserializer(bytes)

        when (val either = Either.Deserialize(deserialize, NoteError).deserialize(deserializer)) {
            is Either.Success -> continuation.resume(either.value)
            is Either.Failure -> continuation.resumeWithException(either.value)
        }

        return true
    }

    override fun abort() {
        val joinHandle = joinHandle ?: throw Error("A Once without a joinHandle is being aborted")

        _abort(joinHandle)
    }

    fun setJoinHandle(handle: Long) {
        joinHandle = handle
    }
}

private class CriticalSection<T> constructor(private val instance: T) {
    private val semaphore: Semaphore = Semaphore(1)

    fun <R> enter(callback: (T) -> R): R {
        semaphore.acquire()
        val res = callback(instance)
        semaphore.release()

        return res
    }
}

class Runtime private constructor() {
    private var futures: CriticalSection<HashMap<Int, Future>> = CriticalSection(HashMap())

    companion object {
        private lateinit var instance: Runtime

        // Return value indicates if the Runtime is already initialized
        fun init(filesDir: String): Either<Boolean, String> {
            if (this::instance.isInitialized) {
                return Either.Success(true)
            }

            System.loadLibrary("reax")

            val either = run(Either.Deserialize(DeUnit, DeString)) {
                _init(
                    BuildConfig.API_URL,
                    BuildConfig.WS_URL,
                    filesDir
                )
            }

            if (either is Either.Failure) {
                return Either.Failure(either.value)
            }

            instance = Runtime()

            thread {
                _initHandler { id, bytes ->
                    Log.d(
                        "Runtime",
                        "received message with id:$id byteLength:${bytes.size}"
                    )

                    val future = instance.futures.enter { futures ->
                        futures[id] ?: throw Error("A message with unknown id is received $id")
                    }

                    if (future.handle(bytes)) {
                        instance.abort(id)
                    }
                }
            }

            return Either.Success(false)
        }

        fun <T> runStream(deserialize: Deserialize<T>, onStart: AsyncStart): Flow<T> =
            instance.runStream(deserialize, onStart)

        suspend fun runOnceUnit(onStart: AsyncStart) = instance.runOnce(DeUnit, onStart)

        suspend fun <T> runOnce(deserialize: Deserialize<T>, onStart: AsyncStart): T =
            instance.runOnce(deserialize, onStart)

        fun <T> run(deserialize: Deserialize<T>, onStart: SyncStart): T {
            return deserialize.deserialize(BincodeDeserializer(onStart()))
        }
    }

    private fun <T> runStream(deserialize: Deserialize<T>, onStart: AsyncStart): Flow<T> =
        callbackFlow {
            val stream = Stream(this, deserialize)

            val id = insertFuture(stream)

            stream.setJoinHandle(onStart(id))

            awaitClose {
                instance.abort(id)
            }
        }

    private suspend fun <T> runOnce(deserialize: Deserialize<T>, onStart: AsyncStart): T =
        suspendCancellableCoroutine { cont ->
            val once = Once(cont, deserialize)

            val id = insertFuture(once)

            once.setJoinHandle(onStart(id))

            cont.invokeOnCancellation {
                abort(id)
            }
        }

    private fun insertFuture(future: Future): Int {
        return futures.enter { futures ->
            var id = Random.nextInt()

            while (futures.contains(id)) {
                id = Random.nextInt()
            }

            futures[id] = future

            id
        }
    }

    private fun abort(id: Int) {
        val future = futures.enter { futures ->
            futures.remove(id) ?: throw Error("Aborting an unknown future $id")
        }

        future.abort()
    }
}

private external fun _init(apiUrl: String, wsUrl: String, storageDir: String): ByteArray
private external fun _initHandler(callback: (streamId: Int, bytes: ByteArray) -> Unit)
private external fun _abort(joinHandle: Long)