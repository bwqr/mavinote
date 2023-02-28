package com.bwqr.mavinote.models

import com.novi.serde.Deserializer

data class Device(
    val id: Int,
    val accountId: Int,
    val pubkey: String,
) {
    companion object {
        fun deserialize(deserializer: Deserializer): Device {
            deserializer.increase_container_depth()

            val device = Device(
                deserializer.deserialize_i32(),
                deserializer.deserialize_i32(),
                deserializer.deserialize_str(),
            )

            deserializer.decrease_container_depth()

            return device
        }
    }
}