import Foundation

class NoteViewModel {
    func folders() async throws -> [Folder] {
        return try await withCheckedThrowingContinuation{ continuation in
            let waitId = Runtime.instance().wait(resume: AsyncWait(continuation) { deserializer in
                try deserializeList(deserializer) { try Folder.deserialize($0) }
            })

            reax_note_folders(waitId)
        }
    }
}
