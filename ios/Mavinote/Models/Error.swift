import Foundation
import Serde

enum NoteError : Error, Deserialize {
    case Mavinote(MavinoteError)
    case Storage(StorageError)
    case Database(String)

    static func deserialize(_ deserializer: Deserializer) throws -> NoteError {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .Mavinote(try MavinoteError.deserialize(deserializer))
        case 1: return .Storage(try StorageError.deserialize(deserializer))
        case 2: return .Database(try deserializer.deserialize_str())
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index for NoteError")
        }
    }

    func handle(_ appState: AppState) {
        switch self {
        case .Mavinote(.NoConnection): appState.emit(BusEvent.ShowMessage("No Internet Connection"))
        default: debugPrint("Unhandled ReaxError \(self)")
        }
    }
}

enum MavinoteError {
    case Unauthorized(Int32?)
    case Message(String)
    case NoConnection
    case UnexpectedResponse
    case Unknown

    static func deserialize(_ deserializer: Deserializer) throws -> MavinoteError {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .NoConnection
        case 1: return .UnexpectedResponse
        case 2: return .Unauthorized(try De.Option<De.I32>.deserialize(deserializer))
        case 3: return .Unknown
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index for MavinoteError")
        }
    }
}

enum StorageError {
    case InvalidState(String)
    case NotMavinoteAccount
    case AccountNotFound
    case AccountEmailUsed
    case FolderNotFound

    static func deserialize(_ deserializer: Deserializer) throws -> StorageError {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .InvalidState(try deserializer.deserialize_str())
        case 1: return .NotMavinoteAccount
        case 2: return .AccountNotFound
        case 3: return .AccountEmailUsed
        case 4: return .FolderNotFound
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index for StorageError")
        }
    }
}
