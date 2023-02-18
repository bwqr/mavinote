package com.bwqr.mavinote.viewmodels

import com.bwqr.mavinote.reax.Runtime

class AccountViewModel {
    companion object {
        suspend fun requestVerification(email: String): String = Runtime.runOnce(
            onNext = { it.deserialize_str() },
            onStart = { _requestVerification(it, email) }
        )

        suspend fun waitVerification(token: String) = Runtime.runUnitOnce {
            _waitVerification(it, token)
        }

        suspend fun addAccount(email: String) = Runtime.runUnitOnce {
            _addAccount(it, email)
        }

        suspend fun publicKey(): String = Runtime.runOnce(
            onNext = { it.deserialize_str() },
            onStart = { _publicKey(it) }
        )
    }
}

private external fun _requestVerification(onceId: Int, email: String): Long
private external fun _waitVerification(onceId: Int, token: String): Long
private external fun _addAccount(onceId: Int, email: String): Long
private external fun _publicKey(onceId: Int): Long