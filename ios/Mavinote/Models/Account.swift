import Serde

struct Account {
    let id: Int32
    let name: String
    let kind: AccountKind

    static func deserialize(_ deserializer: Deserializer) throws -> Account {
        try deserializer.increase_container_depth()

        let account = Account(
            id: try deserializer.deserialize_i32(),
            name: try deserializer.deserialize_str(),
            kind: try AccountKind.deserialize(deserializer)
        )

        try deserializer.decrease_container_depth()

        return account
    }

}

enum AccountKind {
    case Mavinote
    case Local

    static func deserialize(_ deserializer: Deserializer) throws -> AccountKind {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .Mavinote
        case 1: return .Local
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index for HttpError")
        }
    }
}
