import Serde

struct Folder : Identifiable, Deserialize {
    let id: Int32
    let accountId: Int32
    let remoteId: Int32?
    let name: String
    let state: ModelState

    static func deserialize(_ deserializer: Deserializer) throws -> Folder {
        try deserializer.increase_container_depth()

        let folder = Folder(
            id: try deserializer.deserialize_i32(),
            accountId: try deserializer.deserialize_i32(),
            remoteId: try De.Option<De.I32>.deserialize(deserializer),
            name: try deserializer.deserialize_str(),
            state: try ModelState.deserialize(deserializer)
        )

        try deserializer.decrease_container_depth()

        return folder
    }
}
