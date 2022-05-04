package com.bwqr.mavinote.models

import com.novi.bincode.BincodeDeserializer
import com.novi.serde.Deserializer

class TraitHelpers {
    companion object {
        fun<T> deserializeOption(bytes: ByteArray, deserialize: (deserializer: Deserializer) -> T): T? {
            val deserializer = BincodeDeserializer(bytes)
            val tag = deserializer.deserialize_option_tag()

            return if (!tag) {
                null
            } else {
                deserialize(deserializer)
            }
        }

        fun<T> deserializeList(bytes: ByteArray, deserialize: (deserializer: Deserializer) -> T): List<T> {
            val deserializer = BincodeDeserializer(bytes)
            val items = mutableListOf<T>()

            for (index in 0 until deserializer.deserialize_len()) {
                items.add(deserialize(deserializer))
            }

            return items
        }
    }
}