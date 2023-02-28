package com.bwqr.mavinote.viewmodels

import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.Device
import com.bwqr.mavinote.models.Mavinote
import com.bwqr.mavinote.reax.Runtime
import com.bwqr.mavinote.reax.deserializeList
import com.bwqr.mavinote.reax.deserializeOption
import kotlinx.coroutines.flow.Flow

class AccountViewModel {
    companion object {
        fun accounts(): Flow<List<Account>> = Runtime.runStream({
            deserializeList(it) { deserializer ->
                Account.deserialize(deserializer)
            }
        }, { _accounts(it) })

        suspend fun account(accountId: Int): Account? = Runtime.runOnce(
            onNext = { deserializeOption(it) { deserializer -> Account.deserialize(deserializer) } },
            onStart = { _account(it, accountId) }
        )

        suspend fun devices(accountId: Int): List<Device> = Runtime.runOnce(
            onNext = { deserializeList(it) { deserializer -> Device.deserialize(deserializer) } },
            onStart = { _devices(it, accountId) }
        )

        suspend fun requestVerification(email: String): String = Runtime.runOnce(
            onNext = { it.deserialize_str() },
            onStart = { _requestVerification(it, email) }
        )

        suspend fun waitVerification(token: String) = Runtime.runUnitOnce {
            _waitVerification(it, token)
        }

        suspend fun sendVerificationCode(email: String) = Runtime.runUnitOnce {
            _sendVerificationCode(it, email)
        }

        suspend fun signUp(email: String, code: String) = Runtime.runUnitOnce {
            _signUp(it, email, code)
        }

        suspend fun mavinoteAccount(accountId: Int): Mavinote? = Runtime.runOnce({
            deserializeOption(it) { deserializer ->
                Mavinote.deserialize(deserializer)
            }
        }, { _mavinoteAccount(it, accountId) })

        suspend fun addAccount(email: String) = Runtime.runUnitOnce {
            _addAccount(it, email)
        }

        suspend fun removeAccount(accountId: Int) = Runtime.runUnitOnce {
            _removeAccount(it, accountId)
        }

        suspend fun sendAccountCloseCode(accountId: Int) = Runtime.runUnitOnce {
            _sendAccountCloseCode(it, accountId)
        }

        suspend fun closeAccount(accountId: Int, code: String) = Runtime.runUnitOnce {
            _closeAccount(it, accountId, code)
        }

        suspend fun publicKey(): String = Runtime.runOnce(
            onNext = { it.deserialize_str() },
            onStart = { _publicKey(it) }
        )
    }
}

private external fun _accounts(streamId: Int): Long
private external fun _account(onceId: Int, accountId: Int): Long
private external fun _mavinoteAccount(onceId: Int, accountId: Int): Long
private external fun _devices(onceId: Int, accountId: Int): Long
private external fun _requestVerification(onceId: Int, email: String): Long
private external fun _waitVerification(onceId: Int, token: String): Long
private external fun _sendVerificationCode(onceId: Int, email: String): Long
private external fun _signUp(onceId: Int, email: String, code: String): Long
private external fun _addAccount(onceId: Int, email: String): Long
private external fun _removeAccount(onceId: Int, accountId: Int): Long
private external fun _sendAccountCloseCode(onceId: Int, accountId: Int): Long
private external fun _closeAccount(onceId: Int, accountId: Int, code: String): Long
private external fun _publicKey(onceId: Int): Long