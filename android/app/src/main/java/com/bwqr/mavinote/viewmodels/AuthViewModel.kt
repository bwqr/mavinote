package com.bwqr.mavinote.viewmodels

import kotlin.coroutines.suspendCoroutine

class AuthViewModel {
    suspend fun login(email: String, password: String): Result<Unit> {
        return suspendCoroutine { cont ->
            val waitId = Runtime.instance.wait(AsyncWait(cont) { })

            _login(waitId, email, password)
        }
    }

    private external fun _login(waitId: Int, email: String, password: String)
}