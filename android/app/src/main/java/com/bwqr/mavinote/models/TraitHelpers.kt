package com.bwqr.mavinote.models

import com.novi.bincode.BincodeDeserializer
import com.novi.serde.Deserializer

class TraitHelpers {
    companion object {
        fun<T> deserializeOption(deserializer: Deserializer, deserialize: (deserializer: Deserializer) -> T): T? {
            val tag = deserializer.deserialize_option_tag()

            return if (!tag) {
                null
            } else {
                deserialize(deserializer)
            }
        }

        fun<T> deserializeList(deserializer: Deserializer, deserialize: (deserializer: Deserializer) -> T): List<T> {
            val items = mutableListOf<T>()

            for (index in 0 until deserializer.deserialize_len()) {
                items.add(deserialize(deserializer))
            }

            return items
        }
    }
}