package com.bwqr.mavinote.models

import android.util.Log
import com.bwqr.mavinote.Bus
import com.bwqr.mavinote.reax.DeInt
import com.bwqr.mavinote.reax.Deserialize
import com.novi.serde.DeserializationError
import com.novi.serde.Deserializer


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
            else -> {
                Log.e("ReaxError", "Unhandled error, $this")
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
    object Unknown : MavinoteError()

    companion object {
        fun deserialize(deserializer: Deserializer): MavinoteError {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> Unauthorized(DeInt.deserialize(deserializer))
                1 -> Message(deserializer.deserialize_str())
                2 -> NoConnection
                3 -> UnexpectedResponse
                4 -> Unknown
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

    companion object {
        fun deserialize(deserializer: Deserializer): StorageError {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> InvalidState(deserializer.deserialize_str())
                1 -> NotMavinoteAccount
                2 -> AccountNotFound
                3 -> AccountEmailUsed
                4 -> FolderNotFound
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