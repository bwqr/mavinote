package com.bwqr.mavinote.models

import com.novi.serde.Deserializer

data class Note constructor(
    val id: Int,
    val folderId: Int,
    val title: String,
    val text: String,
) {
    companion object {
        fun deserialize(deserializer: Deserializer): Note {
            deserializer.increase_container_depth()

            val note = Note(
                deserializer.deserialize_i32(),
                deserializer.deserialize_i32(),
                deserializer.deserialize_str(),
                deserializer.deserialize_str(),
            )

            deserializer.decrease_container_depth()

            return note
        }
    }

    fun cloneWithText(text: String): Note {
        return Note(
            id,
            folderId,
            title,
            text
        )
    }
}