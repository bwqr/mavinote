import Foundation

struct Folder : Identifiable {
    let id: Int
    let name: String
    
    static func deserialize() -> Folder {
        Folder(id: 1, name: "Merhaba")
    }
}
