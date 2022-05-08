class NoteViewModel {
    func folders() async throws -> [Folder] {
        return try await withCheckedThrowingContinuation { continuation in
            let waitId = Runtime.instance().wait(resume: AsyncWait(continuation) { deserializer in
                try deserializeList(deserializer) { try Folder.deserialize($0) }
            })

            reax_note_folders(waitId)
        }
    }

    func createFolder(_ name: String) async throws -> () {
        return try await withCheckedThrowingContinuation { continuation in
            let waitId = Runtime.instance().wait(resume: AsyncWait(continuation) { deserializer in
            })

            reax_note_create_folder(waitId, name)
        }
    }

    func noteSummaries(_ folderId: Int32) async throws -> [Note] {
        return try await withCheckedThrowingContinuation { continuation in
            let waitId = Runtime.instance().wait(resume: AsyncWait(continuation) { deserializer in
                try deserializeList(deserializer) { try Note.deserialize($0) }
            })

            reax_note_note_summaries(waitId, folderId)
        }
    }

    func note(_ noteId: Int32) async throws -> Note? {
        return try await withCheckedThrowingContinuation { continuation in
            let waitId = Runtime.instance().wait(resume: AsyncWait(continuation) { deserializer in
                try deserializeOption(deserializer) { try Note.deserialize($0) }
            })

            reax_note_note(waitId, noteId)
        }
    }

    func createNote(_ folderId: Int32) async throws -> Int32 {
        return try await withCheckedThrowingContinuation { continuation in
            let waitId = Runtime.instance().wait(resume: AsyncWait(continuation) { deserializer in
                try deserializer.deserialize_i32()
            })

            reax_note_create_note(waitId, folderId)
        }
    }

    func updateNote(_ noteId: Int32, _ text: String) async throws -> () {
        return try await withCheckedThrowingContinuation { continuation in
            let waitId = Runtime.instance().wait(resume: AsyncWait(continuation) { deserializer in
            })

            reax_note_update_note(waitId, noteId, text)
        }
    }
}
