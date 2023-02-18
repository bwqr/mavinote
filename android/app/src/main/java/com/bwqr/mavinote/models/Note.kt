package com.bwqr.mavinote.models

import com.bwqr.mavinote.reax.deserializeOption
import com.novi.serde.DeserializationError
import com.novi.serde.Deserializer

data class Note constructor(
    val id: Int,
    val folderId: Int,
    val remoteId: Int?,
    val commit: Int,
    val name: String,
    val text: String,
    val state: State
) {
    companion object {
        fun deserialize(deserializer: Deserializer): Note {
            deserializer.increase_container_depth()

            val note = Note(
                deserializer.deserialize_i32(),
                deserializer.deserialize_i32(),
                deserializeOption(deserializer) { it.deserialize_i32() },
                deserializer.deserialize_i32(),
                deserializer.deserialize_str(),
                deserializer.deserialize_str(),
                State.deserialize(deserializer)
            )

            deserializer.decrease_container_depth()

            return note
        }
    }
}

enum class State {
    Clean,
    Modified,
    Deleted;

    companion object {
        fun deserialize(deserializer: Deserializer): State {
            val index = deserializer.deserialize_variant_index()

            return when (index) {
                0 -> Clean
                1 -> Modified
                2 -> Deleted
                else -> throw DeserializationError("Unknown variant index for State: $index")
            }
        }
    }
}