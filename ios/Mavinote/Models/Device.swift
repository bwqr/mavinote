import Serde

struct Device: Deserialize, Identifiable {
    let id: Int32
    let accountId: Int32
    let pubkey: String
    let createdAt: String

    static func deserialize(_ deserializer: Deserializer) throws -> Device {
        try deserializer.increase_container_depth()

        let device = Device(
            id: try deserializer.deserialize_i32(),
            accountId: try deserializer.deserialize_i32(),
            pubkey: try deserializer.deserialize_str(),
            createdAt: try deserializer.deserialize_str()
        )

        try deserializer.decrease_container_depth()

        return device
    }
}
