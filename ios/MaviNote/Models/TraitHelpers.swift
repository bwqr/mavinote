import Foundation
import Serde

func deserializeList<T>(_ deserializer: Deserializer, deserialize: (_ deserializer: Deserializer) throws -> T) throws -> [T] {
    var items: [T] = []
    let len = try deserializer.deserialize_len()

    for _ in 0...len {
        items.append(try deserialize(deserializer))
    }

    return items
}
