package com.bwqr.mavinote.models

import com.novi.serde.Deserializer

data class NoteSummary constructor(
    val id: Int,
    val folderId: Int,
    val title: String,
    val summary: String,
) {
    companion object {
        fun deserialize(deserializer: Deserializer): NoteSummary {
            deserializer.increase_container_depth()

            val summary = NoteSummary(
                deserializer.deserialize_i32(),
                deserializer.deserialize_i32(),
                deserializer.deserialize_str(),
                deserializer.deserialize_str(),
            )

            deserializer.decrease_container_depth()

            return summary
        }
    }

}