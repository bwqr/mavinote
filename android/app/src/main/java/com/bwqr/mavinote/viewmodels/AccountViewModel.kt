package com.bwqr.mavinote.viewmodels

import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.Device
import com.bwqr.mavinote.models.Mavinote
import com.bwqr.mavinote.reax.DeBool
import com.bwqr.mavinote.reax.DeList
import com.bwqr.mavinote.reax.DeOption
import com.bwqr.mavinote.reax.DeString
import com.bwqr.mavinote.reax.DeUnit
import com.bwqr.mavinote.reax.Runtime
import kotlinx.coroutines.flow.Flow

class AccountViewModel {
    companion object {
        fun accounts(): Flow<List<Account>> =
            Runtime.runStream(DeList(Account)) { _accounts(it) }

        suspend fun account(accountId: Int): Account? =
            Runtime.runOnce(DeOption(Account)) { _account(it, accountId) }

        suspend fun devices(accountId: Int): List<Device> =
            Runtime.runOnce(DeList(Device)) { _devices(it, accountId) }

        suspend fun addDevice(accountId: Int, fingerprint: String): Unit =
            Runtime.runOnceUnit { _addDevice(it, accountId, fingerprint) }

        suspend fun deleteDevice(accountId: Int, deviceId: Int) =
            Runtime.runOnceUnit { _deleteDevice(it, accountId, deviceId) }

        suspend fun requestVerification(email: String): String =
            Runtime.runOnce(DeString) { _requestVerification(it, email) }

        suspend fun waitVerification(token: String) =
            Runtime.runOnceUnit { _waitVerification(it, token) }

        suspend fun sendVerificationCode(email: String) =
            Runtime.runOnceUnit { _sendVerificationCode(it, email) }

        suspend fun signUp(email: String, code: String) =
            Runtime.runOnceUnit { _signUp(it, email, code) }

        suspend fun mavinoteAccount(accountId: Int): Mavinote? =
            Runtime.runOnce(DeOption(Mavinote)) { _mavinoteAccount(it, accountId) }

        suspend fun addAccount(email: String) = Runtime.runOnceUnit { _addAccount(it, email) }

        suspend fun removeAccount(accountId: Int) =
            Runtime.runOnceUnit { _removeAccount(it, accountId) }

        suspend fun sendAccountCloseCode(accountId: Int) =
            Runtime.runOnceUnit { _sendAccountCloseCode(it, accountId) }

        suspend fun closeAccount(accountId: Int, code: String) =
            Runtime.runOnceUnit { _closeAccount(it, accountId, code) }

        suspend fun publicKey(): String = Runtime.runOnce(DeString) { _publicKey(it) }

        fun listenNotifications(accountId: Int): Flow<Unit> = Runtime.runStream(DeUnit) { _listenNotifications(it, accountId) }

        suspend fun welcomeShown(): Boolean = Runtime.runOnce(DeBool) { _welcomeShown(it) }

        suspend fun updateWelcomeShown(shown: Boolean): Unit = Runtime.runOnceUnit { _updateWelcomeShown(it, shown) }
    }
}

private external fun _accounts(streamId: Int): Long
private external fun _account(onceId: Int, accountId: Int): Long
private external fun _mavinoteAccount(onceId: Int, accountId: Int): Long
private external fun _devices(onceId: Int, accountId: Int): Long
private external fun _addDevice(onceId: Int, accountId: Int, fingerprint: String): Long
private external fun _deleteDevice(onceId: Int, accountId: Int, deviceId: Int): Long
private external fun _requestVerification(onceId: Int, email: String): Long
private external fun _waitVerification(onceId: Int, token: String): Long
private external fun _sendVerificationCode(onceId: Int, email: String): Long
private external fun _signUp(onceId: Int, email: String, code: String): Long
private external fun _addAccount(onceId: Int, email: String): Long
private external fun _removeAccount(onceId: Int, accountId: Int): Long
private external fun _sendAccountCloseCode(onceId: Int, accountId: Int): Long
private external fun _closeAccount(onceId: Int, accountId: Int, code: String): Long
private external fun _publicKey(onceId: Int): Long
private external fun _listenNotifications(streamId: Int, accountId: Int): Long
private external fun _welcomeShown(onceId: Int): Long
private external fun _updateWelcomeShown(onceId: Int, shown: Boolean): Long