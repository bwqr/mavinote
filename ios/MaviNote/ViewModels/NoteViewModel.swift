import Foundation

class NoteViewModel {
    func folders() async throws -> [Folder] {
        return try await withCheckedThrowingContinuation{ continuation in
            let waitId = Runtime.instance().wait(resume: AsyncWait(continuation) {
                [Folder(id: 2, name: "My async folder")]
            })

            reax_note_folders(waitId)
        }
    }
}
