import Foundation
import Serde

protocol Deserialize {
    static func deserialize(_ deserializer: Deserializer) throws -> Self
}

struct DeUnit: Deserialize {
    static func deserialize(_ deserializer: Deserializer) throws -> DeUnit {
        return DeUnit()
    }
}

extension Int32: Deserialize {
    static func deserialize(_ deserializer: Deserializer) throws -> Int32 {
        try deserializer.deserialize_i32()
    }
}

extension Bool: Deserialize {
    static func deserialize(_ deserializer: Deserializer) throws -> Bool {
        try deserializer.deserialize_bool()
    }
}

extension String: Deserialize {
    static func deserialize(_ deserializer: Deserializer) throws -> String {
        try deserializer.deserialize_str()
    }
}

extension Array: Deserialize where Array.Element: Deserialize {
    static func deserialize(_ deserializer: Deserializer) throws -> [Array.Element] {
        var items: [Array.Element] = []
        let len = try deserializer.deserialize_len()

        for _ in 0..<len {
            items.append(try Array.Element.deserialize(deserializer))
        }

        return items
    }
}

extension Optional: Deserialize where Wrapped: Deserialize {
    static func deserialize(_ deserializer: Deserializer) throws -> Optional {
        let tag = try deserializer.deserialize_option_tag()

        if tag {
            return try Wrapped.deserialize(deserializer)
        }

        return nil
    }
}

extension String: Identifiable {
    public typealias ID = Int
    public var id: Int {
        return hash
    }
}
