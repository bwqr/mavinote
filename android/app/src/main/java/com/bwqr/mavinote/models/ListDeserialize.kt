package com.bwqr.mavinote.models

import com.novi.bincode.BincodeDeserializer
import com.novi.serde.Deserializer

fun<T> listDeserialize(bytes: ByteArray, deserialize: (deserializer: Deserializer) -> T): List<T> {
    val deserializer = BincodeDeserializer(bytes)
    val items = mutableListOf<T>()

    for (index in 0 until deserializer.deserialize_len()) {
        items.add(deserialize(deserializer))
    }

    return items
}