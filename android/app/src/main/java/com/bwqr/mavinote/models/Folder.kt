package com.bwqr.mavinote.models

import com.bwqr.mavinote.reax.DeInt
import com.bwqr.mavinote.reax.Deserialize
import com.novi.serde.Deserializer

data class Folder constructor(
    val id: Int,
    val accountId: Int,
    val remoteId: Int?,
    val name: String,
    val state: State
) {
    companion object : Deserialize<Folder> {
        override fun deserialize(deserializer: Deserializer): Folder {
            deserializer.increase_container_depth()

            val folder = Folder(
                deserializer.deserialize_i32(),
                deserializer.deserialize_i32(),
                DeInt.deserialize(deserializer),
                deserializer.deserialize_str(),
                State.deserialize(deserializer)
            )

            deserializer.decrease_container_depth()

            return folder
        }
    }
}