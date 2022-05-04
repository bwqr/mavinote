package com.bwqr.mavinote.models

import com.novi.serde.DeserializationError
import com.novi.serde.Deserializer

class ReaxException constructor(val error: Error) : Throwable()

open class Error {
    companion object {
        fun deserialize(deserializer: Deserializer): Error {
            val index = deserializer.deserialize_variant_index()

            return when (index) {
                0 -> HttpError.deserialize(deserializer)
                else -> throw DeserializationError("Unknown variant index for Error: $index")
            }
        }
    }
}

sealed class HttpError : Error() {
    object NoConnection : HttpError()
    object Unauthorized : HttpError()
    object Unknown : HttpError()

    companion object {
        fun deserialize(deserializer: Deserializer): HttpError {
            val index = deserializer.deserialize_variant_index()

            return when (index) {
                0 -> NoConnection
                1 -> Unauthorized
                2 -> Unknown
                else -> throw DeserializationError("Unknown variant index for HttpError: $index")
            }
        }
    }
}