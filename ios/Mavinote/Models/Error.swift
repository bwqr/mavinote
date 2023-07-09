import Foundation
import Serde

enum NoteError : Error, Deserialize {
    case Mavinote(MavinoteError)
    case Storage(StorageError)
    case Database(String)
    case Crypto(CryptoError)
    case Unreachable(String)
    // This is used by Swift and not returned from Rust
    case TaskCancellation

    static func deserialize(_ deserializer: Deserializer) throws -> NoteError {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .Mavinote(try MavinoteError.deserialize(deserializer))
        case 1: return .Storage(try StorageError.deserialize(deserializer))
        case 2: return .Database(try deserializer.deserialize_str())
        case 3: return .Crypto(try CryptoError.deserialize(deserializer))
        case 4: return .Unreachable(try deserializer.deserialize_str())
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index \(index) for NoteError")
        }
    }
}

enum MavinoteError {
    case Unauthorized(Int32?)
    case Message(String)
    case NoConnection
    case UnexpectedResponse
    case DeviceDeleted(Int32)
    case Unknown(String)

    static func deserialize(_ deserializer: Deserializer) throws -> MavinoteError {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .Unauthorized(try Optional<Int32>.deserialize(deserializer))
        case 1: return .Message(try String.deserialize(deserializer))
        case 2: return .NoConnection
        case 3: return .UnexpectedResponse
        case 4: return .DeviceDeleted(try Int32.deserialize(deserializer))
        case 5: return .Unknown(try String.deserialize(deserializer))
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index \(index) for MavinoteError")
        }
    }
}

enum StorageError {
    case EmailAlreadyExists

    static func deserialize(_ deserializer: Deserializer) throws -> StorageError {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .EmailAlreadyExists
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index \(index) for StorageError")
        }
    }
}

enum CryptoError {
    case Base64Decode
    case InvalidLength
    case Decrypt
    case Encrypt

    static func deserialize(_ deserializer: Deserializer) throws -> CryptoError {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .Base64Decode
        case 1: return .InvalidLength
        case 2: return .Decrypt
        case 3: return .Encrypt
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index \(index) for CryptoError")
        }
    }
}
