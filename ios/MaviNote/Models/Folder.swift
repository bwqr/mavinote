import Foundation
import Serde

struct Folder : Identifiable {
    let id: Int32
    let name: String
    
    static func deserialize(_ deserializer: Deserializer) throws -> Folder {
        try deserializer.increase_container_depth()

        return Folder(id: try deserializer.deserialize_i32(), name: try deserializer.deserialize_str())
    }
}
