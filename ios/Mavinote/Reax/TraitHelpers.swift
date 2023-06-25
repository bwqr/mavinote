import Foundation
import Serde

protocol Deserialize {
    static func deserialize(_ deserializer: Deserializer) throws -> Self
}

protocol IntoInner {
    associatedtype Inner
    
    func into() -> Inner
}

protocol DeserializeInner: Deserialize, IntoInner { }

struct DeInt32: DeserializeInner {
    let val: Int32

    static func deserialize(_ deserializer: Deserializer) throws -> DeInt32 {
        DeInt32(val: try deserializer.deserialize_i32())
    }

    func into() -> Int32 {
        val
    }
}

struct DeString: DeserializeInner {
    let val: String

    static func deserialize(_ deserializer: Deserializer) throws -> DeString {
        DeString(val: try deserializer.deserialize_str())
    }

    func into() -> String {
        val
    }
}

struct DeUnit: DeserializeInner {
    let val: () = ()

    static func deserialize(_ deserializer: Deserializer) throws -> DeUnit {
        DeUnit()
    }

    func into() -> () {
        val
    }
}

struct DeOption<T: Deserialize>: DeserializeInner {
    let val: T?

    static func deserialize(_ deserializer: Deserializer) throws -> DeOption<T> {
        let tag = try deserializer.deserialize_option_tag()

        if tag {
            return DeOption(val: try T.deserialize(deserializer))
        }

        return DeOption(val: nil)
    }

    func into() -> T? {
        val
    }
}

struct DeList<T: Deserialize>: DeserializeInner {
    let val: [T]

    static func deserialize(_ deserializer: Deserializer) throws -> DeList<T> {
        var items: [T] = []
        let len = try deserializer.deserialize_len()

        for _ in 0..<len {
            items.append(try T.deserialize(deserializer))
        }

        return DeList(val: items)
    }

    func into() -> [T] {
        val
    }
}
