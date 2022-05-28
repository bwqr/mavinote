package com.bwqr.mavinote.viewmodels

import kotlinx.coroutines.suspendCancellableCoroutine
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException

class AuthViewModel {
    suspend fun login(email: String, password: String) {
        return suspendCancellableCoroutine { cont ->
            val onceId = Runtime.instance.startOnce(Once(
                onNext = { cont.resume(Unit) },
                onError = { cont.resumeWithException(it) },
                onStart = { _login(it, email, password) }
            ))

            cont.invokeOnCancellation {
                Runtime.instance.abortOnce(onceId)
            }
        }
    }

    private external fun _login(onceId: Int, email: String, password: String): Long
}