package com.bwqr.mavinote.viewmodels

import android.util.Log
import com.bwqr.mavinote.AppConfig
import com.bwqr.mavinote.models.Error
import com.bwqr.mavinote.models.ReaxException
import com.novi.bincode.BincodeDeserializer
import com.novi.serde.Deserializer
import java.util.concurrent.ConcurrentHashMap
import kotlin.concurrent.thread
import kotlin.random.Random

data class Stream constructor(
    val onNext: (deserializer: Deserializer) -> Unit,
    val onError: (error: ReaxException) -> Unit,
    val onComplete: () -> Unit,
    val onStart: (Int) -> Long,
    private var joinHandle: Long? = null
) {
    fun handle(bytes: ByteArray) {
        val deserializer = BincodeDeserializer(bytes)

        when (deserializer.deserialize_variant_index()) {
            0 -> onNext(deserializer)
            1 -> onError(ReaxException(Error.deserialize(deserializer)))
            2 -> onComplete()
        }
    }

    fun start(streamId: Int) {
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

data class Once constructor(
    val onNext: (deserializer: Deserializer) -> Unit,
    val onError: (error: ReaxException) -> Unit,
    val onStart: (onceId: Int) -> Long,
    private var joinHandle: Long? = null
) {
    fun handle(bytes: ByteArray) {
        val deserializer = BincodeDeserializer(bytes)

        when (deserializer.deserialize_variant_index()) {
            0 -> onNext(deserializer)
            1 -> onError(ReaxException(Error.deserialize(deserializer)))
        }
    }

    fun start(onceId: Int) {
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
        lateinit var instance: Runtime

        fun initialize(filesDir: String) {
            if (!this::instance.isInitialized) {
                instance = Runtime(filesDir)
            }
        }
    }

    init {
        System.loadLibrary("reax")

        _init(AppConfig.API_URL, AppConfig.NOTIFY_URL, filesDir)

        thread {
            _initHandler { id, isStream, bytes ->
                Log.d("Runtime", "received message with id:$id isStream:$isStream byteLength:${bytes.size}")

                if (isStream) {
                    streams[id]?.handle(bytes)
                } else {
                    onces[id]?.handle(bytes)
                }
            }
        }
    }

    fun startOnce(once: Once): Int {
        var onceId = Random.nextInt()

        while (onces.contains(onceId)) {
            onceId = Random.nextInt()
        }

        onces[onceId] = once

        once.start(onceId)

        return onceId
    }

    fun startStream(stream: Stream): Int {
        var streamId = Random.nextInt()

        while (streams.contains(streamId)) {
            streamId = Random.nextInt()
        }

        streams[streamId] = stream

        stream.start(streamId)

        return streamId
    }

    fun abortStream(streamId: Int) {
        streams[streamId]?.let { stream -> stream.joinHandle()?.let { _abort(it) } }
        streams.remove(streamId)
    }

    fun abortOnce(onceId: Int) {
        onces[onceId]?.let { once -> once.joinHandle()?.let { _abort(it) } }
        onces.remove(onceId)
    }

    private external fun _init(apiUrl: String, notifyUrl: String, storageDir: String)
    private external fun _initHandler(callback: (streamId: Int, isStream: Boolean, bytes: ByteArray) -> Unit)
    private external fun _abort(joinHandle: Long)
}