import Foundation
import Serde

enum ReaxError : Error {
    case Http(HttpError)
    case Message(String)
    case Database(String)

    static func deserialize(_ deserializer: Deserializer) throws -> ReaxError {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .Http(try HttpError.deserialize(deserializer))
        case 1: return .Message(try deserializer.deserialize_str())
        case 2: return .Database(try deserializer.deserialize_str())
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index for ReaxError")
        }
    }

    func handle(_ appState: AppState) {
        switch self {
        case .Http(.Unauthorized): appState.navigate(Screen.Login)
        case .Http(.NoConnection): appState.emit(BusEvent.NoConnection)
        default: debugPrint("Unhandled ReaxError \(self)")
        }
    }
}

enum HttpError {
    case NoConnection
    case UnexpectedResponse
    case Unauthorized
    case Unknown

    static func deserialize(_ deserializer: Deserializer) throws -> HttpError {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return .NoConnection
        case 1: return .UnexpectedResponse
        case 2: return .Unauthorized
        case 3: return .Unknown
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index for HttpError")
        }
    }
}
