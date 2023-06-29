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

typealias OnStart = (Int) -> Long

interface Future {
    // Return value indicates whether the future is completed or not
    fun handle(bytes: ByteArray): Boolean

    fun abort()
}

class Stream<T, E : Error> constructor(
    private val scope: ProducerScope<T>,
    private val ok: Deserialize<T>,
    private val err: Deserialize<E>
) : Future {
    private var joinHandle: Long? = null

    override fun handle(bytes: ByteArray): Boolean {
        val deserializer = BincodeDeserializer(bytes)

        return when (deserializer.deserialize_variant_index()) {
            0 -> {
                scope.trySend(ok.deserialize(deserializer))
                false
            }

            1 -> {
                scope.cancel("", err.deserialize(deserializer))
                false
            }

            2 -> {
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

class Once<T, E : Error> constructor(
    private val continuation: CancellableContinuation<T>,
    private val ok: Deserialize<T>,
    private val err: Deserialize<E>
) : Future {
    private var joinHandle: Long? = null

    override fun handle(bytes: ByteArray): Boolean {
        val deserializer = BincodeDeserializer(bytes)

        when (deserializer.deserialize_variant_index()) {
            0 -> continuation.resume(ok.deserialize(deserializer))
            1 -> continuation.resumeWithException(err.deserialize(deserializer))
            else -> throw DeserializationError("Unknown variant index for Once")
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

    fun<R> enter(callback: (T) -> R): R {
        semaphore.acquire()
        val res = callback(instance)
        semaphore.release()

        return res
    }
}

class Runtime private constructor(filesDir: String) {
    private var futures: CriticalSection<HashMap<Int, Future>> = CriticalSection(HashMap())

    companion object {
        private lateinit var instance: Runtime

        fun init(filesDir: String) {
            if (!this::instance.isInitialized) {
                instance = Runtime(filesDir)
            }
        }

        fun <T> runStream(deserialize: Deserialize<T>, onStart: OnStart): Flow<T> =
            instance.runStream(deserialize, onStart)

        suspend fun runOnceUnit(onStart: OnStart) = instance.runOnce(DeUnit, onStart)

        suspend fun <T> runOnce(deserialize: Deserialize<T>, onStart: OnStart): T =
            instance.runOnce(deserialize, onStart)
    }

    init {
        _init(BuildConfig.API_URL, BuildConfig.WS_URL, filesDir)

        thread {
            _initHandler { id, bytes ->
                Log.d(
                    "Runtime",
                    "received message with id:$id byteLength:${bytes.size}"
                )

                val future = futures.enter { futures ->
                    futures[id] ?: throw Error("A message with unknown id is received $id")
                }

                if (future.handle(bytes)) {
                    abort(id)
                }
            }
        }
    }

    private fun <T> runStream(deserialize: Deserialize<T>, onStart: OnStart): Flow<T> =
        callbackFlow {
            val stream = Stream(this, deserialize, NoteError.Companion)

            val id = insertFuture(stream)

            stream.setJoinHandle(onStart(id))

            awaitClose {
                instance.abort(id)
            }
        }

    private suspend fun <T> runOnce(deserialize: Deserialize<T>, onStart: OnStart): T =
        suspendCancellableCoroutine { cont ->
            val once = Once(cont, deserialize, NoteError.Companion)

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

private external fun _init(apiUrl: String, wsUrl: String, storageDir: String)
private external fun _initHandler(callback: (streamId: Int, bytes: ByteArray) -> Unit)
private external fun _abort(joinHandle: Long)