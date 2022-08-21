import Serde

struct Account : Identifiable {
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

enum AccountKind : String, CaseIterable {
    case Mavinote
    case Local

    static func deserialize(_ deserializer: Deserializer) throws -> AccountKind {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .Mavinote
        case 1: return .Local
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index for AccountKind")
        }
    }
}

struct Mavinote {
    let email: String
    let token: String

    static func deserialize(_ deserializer: Deserializer) throws -> Mavinote {
        try deserializer.increase_container_depth()

        let mavinote = Mavinote(
            email: try deserializer.deserialize_str(),
            token: try deserializer.deserialize_str()
        )

        try deserializer.decrease_container_depth()

        return mavinote

    }
}
