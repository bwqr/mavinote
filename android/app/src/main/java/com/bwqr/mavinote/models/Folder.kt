package com.bwqr.mavinote.models

import com.novi.serde.Deserializer

data class Folder constructor(
    val id: Int,
    val name: String,
) {
    companion object {
        fun deserialize(deserializer: Deserializer): Folder {
            deserializer.increase_container_depth()

            val folder = Folder(
                deserializer.deserialize_i32(),
                deserializer.deserialize_str(),
            )

            deserializer.decrease_container_depth()

            return folder
        }
    }
}