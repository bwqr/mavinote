import Serde

struct Note : Identifiable {
    let id: Int32
    let folderId: Int32
    let title: String
    let text: String

    static func deserialize(_ deserializer: Deserializer) throws -> Note {
        try deserializer.increase_container_depth()

        let note = Note(
            id: try deserializer.deserialize_i32(),
            folderId: try deserializer.deserialize_i32(),
            title: try deserializer.deserialize_str(),
            text: try deserializer.deserialize_str()
        )

        try deserializer.decrease_container_depth()

        return note
    }
}
