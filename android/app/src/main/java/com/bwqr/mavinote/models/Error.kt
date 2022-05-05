package com.bwqr.mavinote.models

import com.bwqr.mavinote.viewmodels.Bus
import com.bwqr.mavinote.viewmodels.BusEvent
import com.novi.serde.DeserializationError
import com.novi.serde.Deserializer

class ReaxException constructor(val error: Error) : Throwable() {
    fun handle() {
        when (error) {
            is HttpError.NoConnection -> Bus.emit(BusEvent.DisplayNoInternetWarning)
            is HttpError.Unauthorized -> Bus.emit(BusEvent.RequireAuthorization)
        }
    }
}

open class Error {
    companion object {
        fun deserialize(deserializer: Deserializer): Error {
            val index = deserializer.deserialize_variant_index()

            return when (index) {
                0 -> HttpError.deserialize(deserializer)
                1 -> Message.deserialize(deserializer)
                2 -> Database()
                else -> throw DeserializationError("Unknown variant index for Error: $index")
            }
        }
    }
}

sealed class HttpError : Error() {
    object NoConnection : HttpError()
    object UnexpectedResponse: HttpError()
    object Unauthorized : HttpError()
    object Unknown : HttpError()

    companion object {
        fun deserialize(deserializer: Deserializer): HttpError {
            val index = deserializer.deserialize_variant_index()

            return when (index) {
                0 -> NoConnection
                1 -> UnexpectedResponse
                2 -> Unauthorized
                3 -> Unknown
                else -> throw DeserializationError("Unknown variant index for HttpError: $index")
            }
        }
    }
}

data class Message(val message: String) : Error() {
    companion object {
        fun deserialize(deserializer: Deserializer): Message {
            return Message(deserializer.deserialize_str())
        }
    }
}

class Database : Error()