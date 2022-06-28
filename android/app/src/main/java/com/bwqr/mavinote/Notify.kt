package com.bwqr.mavinote

import com.bwqr.mavinote.viewmodels.Once
import com.bwqr.mavinote.viewmodels.Runtime
import com.bwqr.mavinote.viewmodels.Stream
import kotlinx.coroutines.cancel
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException

class Notify {
    suspend fun start(): Unit = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(Unit) },
            onError = { cont.resumeWithException(it) },
            onStart = { _start(it) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    suspend fun stop(): Unit = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(Unit) },
            onError = { cont.resumeWithException(it) },
            onStart = { _stop(it) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    fun connected(): Flow<Boolean> = callbackFlow {
        val streamId = Runtime.instance.startStream(Stream(
            onNext = { deserializer ->
                trySend(deserializer.deserialize_bool())
            },
            onError = { cancel("", it) },
            onStart = { _connected(it) },
            onComplete = { channel.close() }
        ))

        awaitClose {
            Runtime.instance.abortStream(streamId)
        }
    }

    private external fun _start(onceId: Int): Long
    private external fun _stop(onceId: Int): Long
    private external fun _connected(streamId: Int): Long
}