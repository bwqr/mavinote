import Serde

struct Note : Identifiable, Deserialize {
    let id: Int32
    let folderId: Int32
    let remoteId: Int32?
    let commit: Int32
    let name: String
    let text: String
    let state: ModelState

    static func deserialize(_ deserializer: Deserializer) throws -> Note {
        try deserializer.increase_container_depth()

        let note = Note(
            id: try deserializer.deserialize_i32(),
            folderId: try deserializer.deserialize_i32(),
            remoteId: try De.Option<De.I32>.deserialize(deserializer),
            commit: try deserializer.deserialize_i32(),
            name: try deserializer.deserialize_str(),
            text: try deserializer.deserialize_str(),
            state: try ModelState.deserialize(deserializer)
        )

        try deserializer.decrease_container_depth()

        return note
    }
}

enum ModelState: Deserialize {
    case Clean
    case Modified
    case Deleted

    static func deserialize(_ deserializer: Deserializer) throws -> ModelState {
        try deserializer.increase_container_depth()

        let index = try deserializer.deserialize_variant_index()

        let state: ModelState

        switch index {
        case 0: state = Clean
        case 1: state = Modified
        case 2: state = Deleted
        default: throw DeserializationError.invalidInput(issue: "Invalid variant for State \(index)")
        }

        try deserializer.decrease_container_depth()

        return state
    }

}
