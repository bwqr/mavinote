package com.bwqr.mavinote.reax

import com.novi.serde.Deserializer

interface Deserialize<T> {
    fun deserialize(deserializer: Deserializer): T
}

class DeOption<T> constructor(private val wrapped: Deserialize<T>): Deserialize<T?> {
    override fun deserialize(deserializer: Deserializer): T? {
        val tag = deserializer.deserialize_option_tag()

        if (tag) {
            return wrapped.deserialize(deserializer)
        }

        return null
    }
}

class DeList<T> constructor(private val wrapped: Deserialize<T>): Deserialize<List<T>> {
    override fun deserialize(deserializer: Deserializer): List<T> {
    val items = mutableListOf<T>()

    for (index in 0 until deserializer.deserialize_len()) {
        items.add(wrapped.deserialize(deserializer))
    }

    return items
    }

}

object DeUnit: Deserialize<Unit> {
    override fun deserialize(deserializer: Deserializer) { }
}

object DeString: Deserialize<String> {
    override fun deserialize(deserializer: Deserializer): String {
        return deserializer.deserialize_str()
    }
}

object DeInt: Deserialize<Int> {
    override fun deserialize(deserializer: Deserializer): Int {
        return deserializer.deserialize_i32()
    }
}