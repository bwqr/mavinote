import Foundation
import Serde

protocol DeserializeInto<Target> {
    associatedtype Target

    static func deserialize(_ deserializer: Deserializer) throws -> Target
}

protocol Deserialize: DeserializeInto<Self> {
    static func deserialize(_ deserializer: Deserializer) throws -> Self
}

struct De {
    struct I32: DeserializeInto {
        static func deserialize(_ deserializer: Deserializer) throws -> Int32 {
            try deserializer.deserialize_i32()
        }
    }

    struct Str: DeserializeInto {
        static func deserialize(_ deserializer: Deserializer) throws -> String {
            try deserializer.deserialize_str()
        }
    }

    struct Option<T: DeserializeInto>: DeserializeInto {
        static func deserialize(_ deserializer: Deserializer) throws -> T.Target? {
            let tag = try deserializer.deserialize_option_tag()

            if tag {
                return try T.deserialize(deserializer)
            }

            return nil
        }
    }

    struct Unit: DeserializeInto {
        static func deserialize(_ deserializer: Deserializer) throws -> () { }
    }

    struct List<T: DeserializeInto>: DeserializeInto {
        static func deserialize(_ deserializer: Deserializer) throws -> [T.Target] {
            var items: [T.Target] = []
            let len = try deserializer.deserialize_len()

            for _ in 0..<len {
                items.append(try T.deserialize(deserializer))
            }

            return items
        }
    }
}
