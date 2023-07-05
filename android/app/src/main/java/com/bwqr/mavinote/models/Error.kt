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
    class Internal(override val message: String) : MavinoteError()
    object Unknown : MavinoteError()

    companion object {
        fun deserialize(deserializer: Deserializer): MavinoteError {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> Unauthorized(DeOption(DeInt).deserialize(deserializer))
                1 -> Message(deserializer.deserialize_str())
                2 -> NoConnection
                3 -> UnexpectedResponse
                4 -> DeviceDeleted(deserializer.deserialize_i32())
                5 -> Internal(deserializer.deserialize_str())
                6 -> Unknown
                else -> throw DeserializationError("Unknown variant index for MavinoteError: $index")
            }
        }
    }
}

sealed class StorageError : NoteError() {
    class InvalidState(override val message: String) : StorageError()
    object NotMavinoteAccount : StorageError()
    object AccountNotFound : StorageError()
    object AccountEmailUsed : StorageError()
    object FolderNotFound : StorageError()
    object NoteNotFound: StorageError()

    companion object {
        fun deserialize(deserializer: Deserializer): StorageError {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> InvalidState(deserializer.deserialize_str())
                1 -> NotMavinoteAccount
                2 -> AccountNotFound
                3 -> AccountEmailUsed
                4 -> FolderNotFound
                5 -> NoteNotFound
                else -> throw DeserializationError("Unknown variant index for MavinoteError: $index")
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