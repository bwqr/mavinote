package com.bwqr.mavinote.reax

import android.util.Log
import com.bwqr.mavinote.BuildConfig
import com.bwqr.mavinote.models.Error
import com.novi.bincode.BincodeDeserializer
import com.novi.serde.Deserializer
import kotlinx.coroutines.cancel
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.suspendCancellableCoroutine
import java.util.concurrent.ConcurrentHashMap
import kotlin.concurrent.thread
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException
import kotlin.random.Random

class Stream constructor(
    val onNext: (deserializer: Deserializer) -> Unit,
    val onError: (error: Error) -> Unit,
    val onComplete: () -> Unit,
    val onStart: (Int) -> Long,
) {
    private var joinHandle: Long? = null

    fun handle(bytes: ByteArray) {
        val deserializer = BincodeDeserializer(bytes)

        when (deserializer.deserialize_variant_index()) {
            0 -> onNext(deserializer)
            1 -> onError(Error.deserialize(deserializer))
            2 -> onComplete()
        }
    }

    fun run(streamId: Int) {
        if (joinHandle != null) {
            Log.e("Stream", "Stream started more than once")
            return
        }

        joinHandle = onStart(streamId)
    }

    fun joinHandle(): Long? {
        return joinHandle
    }
}

class Once constructor(
    val onNext: (deserializer: Deserializer) -> Unit,
    val onError: (error: Error) -> Unit,
    val onStart: (onceId: Int) -> Long,
) {
    private var joinHandle: Long? = null

    fun handle(bytes: ByteArray) {
        val deserializer = BincodeDeserializer(bytes)

        when (deserializer.deserialize_variant_index()) {
            0 -> onNext(deserializer)
            1 -> onError(Error.deserialize(deserializer))
        }
    }

    fun run(onceId: Int) {
        if (joinHandle != null) {
            Log.e("Once", "Once started more than once")
            return
        }

        joinHandle = onStart(onceId)
    }

    fun joinHandle(): Long? {
        return joinHandle
    }
}

class Runtime private constructor(filesDir: String) {
    private var streams: ConcurrentHashMap<Int, Stream> = ConcurrentHashMap()
    private var onces: ConcurrentHashMap<Int, Once> = ConcurrentHashMap()

    companion object {
        private lateinit var instance: Runtime

        fun init(filesDir: String) {
            if (!this::instance.isInitialized) {
                instance = Runtime(filesDir)
            }
        }

        suspend fun runUnitOnce(onStart: (onceId: Int) -> Long): Unit = runOnce({ }, onStart)

        suspend fun <T> runOnce(
            onNext: (deserializer: Deserializer) -> T,
            onStart: (onceId: Int) -> Long
        ): T = suspendCancellableCoroutine { cont ->
            val once = Once(
                onNext = { cont.resume(onNext(it)) },
                onError = { cont.resumeWithException(it) },
                onStart
            )

            val onceId = instance.insertOnce(once)

            once.run(onceId)

            cont.invokeOnCancellation {
                instance.abortOnce(onceId)
            }
        }

        fun <T> runStream(
            onNext: (deserializer: Deserializer) -> T,
            onStart: (streamId: Int) -> Long
        ): Flow<T> = callbackFlow {
            val stream = Stream(
                onNext = { deserializer -> trySend(onNext(deserializer)) },
                onError = { cancel("", it) },
                onStart = onStart,
                onComplete = { channel.close() }
            )

            val streamId = instance.insertStream(stream)

            stream.run(streamId)

            awaitClose {
                instance.abortStream(streamId)
            }
        }
    }

    init {
        _init(BuildConfig.API_ENDPOINT, filesDir)

        thread {
            _initHandler { id, isStream, bytes ->
                Log.d(
                    "Runtime",
                    "received message with id:$id isStream:$isStream byteLength:${bytes.size}"
                )

                if (isStream) {
                    streams[id]?.handle(bytes)
                } else {
                    onces[id]?.handle(bytes)
                }
            }
        }
    }

    private fun insertOnce(once: Once): Int {
        var onceId = Random.nextInt()

        while (onces.contains(onceId)) {
            onceId = Random.nextInt()
        }

        onces[onceId] = once

        return onceId
    }

    private fun insertStream(stream: Stream): Int {
        var streamId = Random.nextInt()

        while (streams.contains(streamId)) {
            streamId = Random.nextInt()
        }

        streams[streamId] = stream

        return streamId
    }

    private fun abortStream(streamId: Int) {
        streams[streamId]?.let { stream -> stream.joinHandle()?.let { _abort(it) } }
        streams.remove(streamId)
    }

    private fun abortOnce(onceId: Int) {
        onces[onceId]?.let { once -> once.joinHandle()?.let { _abort(it) } }
        onces.remove(onceId)
    }
}

private external fun _init(apiUrl: String, storageDir: String)
private external fun _initHandler(callback: (streamId: Int, isStream: Boolean, bytes: ByteArray) -> Unit)
private external fun _abort(joinHandle: Long)