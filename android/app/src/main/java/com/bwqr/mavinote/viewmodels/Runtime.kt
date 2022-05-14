package com.bwqr.mavinote.viewmodels

import android.util.Log
import com.bwqr.mavinote.AppConfig
import com.bwqr.mavinote.models.Error
import com.bwqr.mavinote.models.ReaxException
import com.novi.bincode.BincodeDeserializer
import com.novi.serde.Deserializer
import java.util.concurrent.ConcurrentHashMap
import kotlin.concurrent.thread
import kotlin.coroutines.Continuation
import kotlin.coroutines.resume
import kotlin.random.Random

data class AsyncWait<T> constructor(
    val continuation: Continuation<Result<T>>,
    val deserialize: (deserializer: Deserializer) -> T,
) {
    fun handle(ok: Boolean, bytes: ByteArray) {
        val deserializer = BincodeDeserializer(bytes)

        if (ok) {
            continuation.resume(Result.success(deserialize(deserializer)))
        } else {
            continuation.resume(Result.failure(ReaxException(Error.deserialize(deserializer))))
        }
    }
}

class Runtime private constructor(filesDir: String) {
    private var waits: ConcurrentHashMap<Int, AsyncWait<*>> = ConcurrentHashMap()

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

        _init(AppConfig.APP_NAME, AppConfig.API_URL, filesDir)

        thread {
            _initHandler { waitId: Int, ok: Boolean, bytes: ByteArray ->
                Log.d("Runtime", "received message $waitId ${bytes.size}")
                waits[waitId]?.handle(ok, bytes)

                waits.remove(waitId)
            }
        }
    }

    fun<T> wait(asyncWait: AsyncWait<T>): Int {
        var waitId = Random.nextInt()

        while (waits.contains(waitId)) {
            waitId = Random.nextInt()
        }

        waits[waitId] = asyncWait

        return waitId
    }

    private external fun _init(appName: String, apiUrl: String, storageDir: String)
    private external fun _initHandler(callback: (waitId: Int, ok: Boolean, bytes: ByteArray) -> Unit)
}