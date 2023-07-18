package com.bwqr.mavinote.reax

import com.novi.serde.Deserializer

open class Either<T, E> {
    class Success<T, E> constructor(val value: T) : Either<T, E>()

    class Failure<T, E> constructor(val value: E) : Either<T, E>()

    class Deserialize<T, E> constructor(
        private val success: com.bwqr.mavinote.reax.Deserialize<T>, private val failure: com.bwqr.mavinote.reax.Deserialize<E>
    ) : com.bwqr.mavinote.reax.Deserialize<Either<T, E>> {
        override fun deserialize(deserializer: Deserializer): Either<T, E> {
            return when (val index = deserializer.deserialize_variant_index()) {
                0 -> Success(success.deserialize(deserializer))
                1 -> Failure(failure.deserialize(deserializer))
                else -> throw Error("Unknown variant for Either $index")
            }
        }
    }
}