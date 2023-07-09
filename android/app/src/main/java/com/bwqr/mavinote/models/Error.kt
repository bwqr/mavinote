package com.bwqr.mavinote.models

import android.util.Log
import com.bwqr.mavinote.Bus
import com.bwqr.mavinote.BusEvent
import com.bwqr.mavinote.reax.DeInt
import com.bwqr.mavinote.reax.DeOption
import com.bwqr.mavinote.reax.Deserialize
import com.bwqr.mavinote.viewmodels.AccountViewModel
import com.novi.serde.DeserializationError
import com.novi.serde.Deserializer
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch


open class NoteError : Error() {
    companion object: Deserialize<NoteError> {
        override fun deserialize(deserializer: Deserializer): NoteError {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> MavinoteError.deserialize(deserializer)
                1 -> StorageError.deserialize(deserializer)
                2 -> DatabaseError.deserialize(deserializer)
                3 -> CryptoError.deserialize(deserializer)
                4 -> UnreachableError.deserialize(deserializer)
                else -> throw DeserializationError("Unknown variant index for Error: $index")
            }
        }
    }

    fun handle() {
        when (this) {
            is MavinoteError.NoConnection -> Bus.message("No Internet Connection")
            is MavinoteError.DeviceDeleted -> {
                val accountId = this.accountId
                GlobalScope.launch {
                    try {
                        AccountViewModel.removeAccount(accountId)
                    } catch (e: NoteError) {
                        if (e is MavinoteError.DeviceDeleted) {
                            Log.e("NoteError", "Nested DeviceDeleted error is encountered")
                            Bus.emit(BusEvent.ShowMessage("Nested DeviceDeleted error is encountered"))
                        } else {
                            e.handle()
                        }
                    }
                }
            }
            else -> {
                Log.e("NoteError", "Unhandled error, $this")
                Bus.message(this.toString())
            }
        }
    }
}

sealed class MavinoteError : NoteError() {
    class Unauthorized(val accountId: Int?) : MavinoteError()
    class Message(override val message: String) : MavinoteError()
    object NoConnection : MavinoteError()
    object UnexpectedResponse : MavinoteError()
    class DeviceDeleted(val accountId: Int) : MavinoteError()
    class Unknown(override val message: String) : MavinoteError()

    companion object {
        fun deserialize(deserializer: Deserializer): MavinoteError {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> Unauthorized(DeOption(DeInt).deserialize(deserializer))
                1 -> Message(deserializer.deserialize_str())
                2 -> NoConnection
                3 -> UnexpectedResponse
                4 -> DeviceDeleted(deserializer.deserialize_i32())
                5 -> Unknown(deserializer.deserialize_str())
                else -> throw DeserializationError("Unknown variant index for MavinoteError: $index")
            }
        }
    }
}

sealed class StorageError : NoteError() {
    object EmailAlreadyExists : StorageError()

    companion object {
        fun deserialize(deserializer: Deserializer): StorageError {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> EmailAlreadyExists
                else -> throw DeserializationError("Unknown variant index for StorageError: $index")
            }
        }
    }
}

data class DatabaseError(override val message: String) : NoteError() {
    companion object {
        fun deserialize(deserializer: Deserializer): DatabaseError {
            return DatabaseError(deserializer.deserialize_str())
        }
    }
}

sealed class CryptoError: NoteError() {
    object Base64Decode : CryptoError()
    object InvalidLength : CryptoError()
    object Decrypt : CryptoError()
    object Encrypt : CryptoError()

    companion object {
        fun deserialize(deserializer: Deserializer): CryptoError {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> Base64Decode
                1 -> InvalidLength
                2 -> Decrypt
                3 -> Encrypt
                else -> throw DeserializationError("Unknown variant index for CryptoError: $index")
            }
        }
    }
}

data class UnreachableError(override val message: String) : NoteError() {
    companion object {
        fun deserialize(deserializer: Deserializer): UnreachableError {
            return UnreachableError(deserializer.deserialize_str())
        }
    }
}