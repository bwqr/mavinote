package com.bwqr.mavinote.models

import android.util.Log
import com.bwqr.mavinote.Bus
import com.bwqr.mavinote.BusEvent
import com.bwqr.mavinote.reax.deserializeOption
import com.novi.serde.DeserializationError
import com.novi.serde.Deserializer


open class Error : Throwable() {
    companion object {
        fun deserialize(deserializer: Deserializer): Error {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> HttpError.deserialize(deserializer)
                1 -> Message.deserialize(deserializer)
                2 -> Database.deserialize(deserializer)
                else -> throw DeserializationError("Unknown variant index for Error: $index")
            }
        }
    }

    fun handle() {
        when (this) {
            is HttpError.NoConnection -> Bus.emit(BusEvent.DisplayNoInternetWarning)
            is HttpError.Unauthorized -> this.accountId?.let { Bus.emit(BusEvent.RequireAuthorization(it)) }
            else -> Log.e("ReaxError", "Unhandled error, $this")
        }
    }
}

sealed class HttpError : Error() {
    object NoConnection : HttpError()
    object UnexpectedResponse : HttpError()
    class Unauthorized(val accountId: Int?) : HttpError()
    object Unknown : HttpError()

    companion object {
        fun deserialize(deserializer: Deserializer): HttpError {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> NoConnection
                1 -> UnexpectedResponse
                2 -> Unauthorized(deserializeOption(deserializer) {
                    it.deserialize_i32()
                })
                3 -> Unknown
                else -> throw DeserializationError("Unknown variant index for HttpError: $index")
            }
        }
    }
}

data class Message(override val message: String) : Error() {
    companion object {
        fun deserialize(deserializer: Deserializer): Message {
            return Message(deserializer.deserialize_str())
        }
    }
}

data class Database(override val message: String) : Error() {
    companion object {
        fun deserialize(deserializer: Deserializer): Database {
            return Database(deserializer.deserialize_str())
        }
    }
}