class NoteViewModel {
    static func initialize() {
        reax_note_init()
    }

    static func accounts() -> AsyncStream<Result<[Account], ReaxError>> {
        return Runtime.runStream {
            try deserializeList($0) { try Account.deserialize($0) }
        } _: {
            reax_note_accounts($0)
        }
    }

    static func account(_ accountId: Int32) async throws -> Account? {
        return try await Runtime.runOnce {
            try deserializeOption($0) { try Account.deserialize($0) }
        } _: {
            reax_note_account($0, accountId)
        }
    }

    static func mavinoteAccount(_ accountId: Int32) async throws -> Mavinote? {
        return try await Runtime.runOnce {
            try deserializeOption($0) { try Mavinote.deserialize($0) }
        } _: {
            reax_note_mavinote_account($0, accountId)
        }
    }

    static func addAccount(_ name: String, _ email: String, _ password: String, _ createAccount: Bool) async throws -> () {
        return try await Runtime.runUnitOnce {
            reax_note_add_account($0, name, email, password, createAccount)
        }
    }

    static func deleteAccount(_ accountId: Int32) async throws -> () {
        return try await Runtime.runUnitOnce {
            reax_note_delete_account($0, accountId)
        }
    }

    static func sync() async throws -> () {
        return try await Runtime.runUnitOnce {
            reax_note_sync($0)
        }
    }

    static func folders() -> AsyncStream<Result<[Folder], ReaxError>> {
        return Runtime.runStream {
            try deserializeList($0) { try Folder.deserialize($0) }
        } _: {
            reax_note_folders($0)
        }
    }

    static func folder(_ folderId: Int32) async throws -> Folder? {
        return try await Runtime.runOnce {
            try deserializeOption($0) { try Folder.deserialize($0) }
        } _: {
            reax_note_folder($0, folderId)
        }
    }

    static func createFolder(_ accountId: Int32, _ name: String) async throws -> () {
        return try await Runtime.runUnitOnce {
            reax_note_create_folder($0, accountId, name)
        }
    }

    static func deleteFolder(_ folderId: Int32) async throws -> () {
        return try await Runtime.runUnitOnce {
            reax_note_delete_folder($0, folderId)
        }
    }

    static func authorizeAccount(_ accountId: Int32, password: String) async throws -> () {
        return try await Runtime.runUnitOnce {
            reax_note_authorize_account($0, accountId, password)
        }
    }

    static func noteSummaries(_ folderId: Int32) -> AsyncStream<Result<[Note], ReaxError>> {
        return Runtime.runStream {
            try deserializeList($0) { try Note.deserialize($0) }
        } _: {
            reax_note_note_summaries($0, folderId)
        }
    }

    static func note(_ noteId: Int32) async throws -> Note? {
        return try await Runtime.runOnce {
            try deserializeOption($0) { try Note.deserialize($0) }
        } _: {
            reax_note_note($0, noteId)
        }
    }

    static func createNote(_ folderId: Int32, _ text: String) async throws -> Int32 {
        return try await Runtime.runOnce {
            try $0.deserialize_i32()
        } _: {
            reax_note_create_note($0, folderId, text)
        }
    }

    static func updateNote(_ noteId: Int32, _ text: String) async throws -> () {
        return try await Runtime.runUnitOnce {
            reax_note_update_note($0, noteId, text)
        }
    }

    static func deleteNote(_ noteId: Int32) async throws -> () {
        return try await Runtime.runUnitOnce {
            reax_note_delete_note($0, noteId)
        }
    }
}
