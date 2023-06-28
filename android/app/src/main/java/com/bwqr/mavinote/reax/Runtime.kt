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
import java.util.concurrent.ConcurrentHashMap
import kotlin.concurrent.thread
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException
import kotlin.random.Random

typealias OnceStart = (Int) -> Unit
typealias StreamStart = (Int) -> Long

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
    override fun handle(bytes: ByteArray): Boolean {
        val deserializer = BincodeDeserializer(bytes)

        when (deserializer.deserialize_variant_index()) {
            0 -> continuation.resume(ok.deserialize(deserializer))
            1 -> continuation.resumeWithException(err.deserialize(deserializer))
            else -> throw DeserializationError("Unknown variant index for Once")
        }

        return true
    }

    // Once does not support abort
    override fun abort() {}
}

class Runtime private constructor(filesDir: String) {
    private var futures: ConcurrentHashMap<Int, Future> = ConcurrentHashMap()

    companion object {
        private lateinit var instance: Runtime

        fun init(filesDir: String) {
            if (!this::instance.isInitialized) {
                instance = Runtime(filesDir)
            }
        }

        fun <T> runStream(deserialize: Deserialize<T>, onStart: StreamStart): Flow<T> =
            instance.runStream(deserialize, onStart)

        suspend fun runOnceUnit(onStart: OnceStart) = instance.runOnce(DeUnit, onStart)

        suspend fun <T> runOnce(deserialize: Deserialize<T>, onStart: OnceStart): T =
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

                val future = futures[id] ?: throw Error("A message with unknown id is received $id")

                if (future.handle(bytes)) {
                    abort(id)
                }
            }
        }
    }

    /**
     * The order of statements in callbackFlow is important. This order enforces
     * the presence of stream in the futures if a call made to onStart directly
     * triggers the handler thread.
     */
    private fun <T> runStream(deserialize: Deserialize<T>, onStart: StreamStart): Flow<T> =
        callbackFlow {
            val id = generateId()
            val stream = Stream(this, deserialize, NoteError.Companion)
            futures[id] = stream

            stream.setJoinHandle(onStart(id))

            awaitClose {
                instance.abort(id)
            }
        }

    private suspend fun <T> runOnce(deserialize: Deserialize<T>, onStart: OnceStart): T =
        suspendCancellableCoroutine { cont ->
            val id = generateId()

            futures[id] = Once(cont, deserialize, NoteError.Companion)

            onStart(id)
        }

    private fun generateId(): Int {
        var id = Random.nextInt()

        while (futures.contains(id)) {
            id = Random.nextInt()
        }

        return id
    }

    private fun abort(id: Int) {
        futures.remove(id)?.abort()
    }
}

private external fun _init(apiUrl: String, wsUrl: String, storageDir: String)
private external fun _initHandler(callback: (streamId: Int, bytes: ByteArray) -> Unit)
private external fun _abort(joinHandle: Long)