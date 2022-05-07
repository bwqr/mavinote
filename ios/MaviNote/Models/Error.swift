import Foundation
import Serde

class ReaxError : Error {
    static func deserialize(_ deserializer: Deserializer) throws -> ReaxError {
        let index = try deserializer.deserialize_variant_index()

        switch index {
        case 0: return HttpError()
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index for ReaxError")
        }
    }
}

class HttpError : ReaxError { }
