package com.bwqr.mavinote.models

import com.novi.serde.DeserializationError
import com.novi.serde.Deserializer

data class Account(
    val id: Int,
    val name: String,
    val kind: AccountKind,
) {
    companion object {
        fun deserialize(deserializer: Deserializer): Account {
            deserializer.increase_container_depth()

            val account = Account(
                deserializer.deserialize_i32(),
                deserializer.deserialize_str(),
                AccountKind.deserialize(deserializer)
            )

            deserializer.decrease_container_depth()

            return account
        }
    }
}

enum class AccountKind {
    Mavinote,
    Local;

    companion object {
        fun deserialize(deserializer: Deserializer): AccountKind {
            val index = deserializer.deserialize_variant_index()

            return when (index) {
                0 -> Mavinote
                1 -> Local
                else -> throw DeserializationError("Unknown variant index for AccountKind: $index")
            }
        }
    }
}