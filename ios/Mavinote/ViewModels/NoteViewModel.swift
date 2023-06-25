class NoteViewModel {
    static func initialize() {
        reax_note_init()
    }

    static func sync() async throws {
        return try await Runtime.runOnce(DeUnit.self) { reax_note_sync($0) }
    }

    static func folders() -> AsyncStream<Result<[Folder], NoteError>> {
        return Runtime.runStream(DeList<Folder>.self) { reax_note_folders($0) }
    }

    static func folder(_ folderId: Int32) async throws -> Folder? {
        return try await Runtime.runOnce(DeOption<Folder>.self) { reax_note_folder($0, folderId) }
    }

    static func createFolder(_ accountId: Int32, _ name: String) async throws {
        return try await Runtime.runOnce(DeUnit.self) { reax_note_create_folder($0, accountId, name) }
    }

    static func deleteFolder(_ folderId: Int32) async throws {
        return try await Runtime.runOnce(DeUnit.self) { reax_note_delete_folder($0, folderId) }
    }

    static func notes(_ folderId: Int32) -> AsyncStream<Result<[Note], NoteError>> {
        return Runtime.runStream(DeList<Note>.self) { reax_note_note_summaries($0, folderId) }
    }

    static func note(_ noteId: Int32) async throws -> Note? {
        return try await Runtime.runOnce(DeOption<Note>.self) { reax_note_note($0, noteId) }
    }

    static func createNote(_ folderId: Int32, _ text: String) async throws {
        return try await Runtime.runOnce(DeUnit.self) { reax_note_create_note($0, folderId, text) }
    }

    static func updateNote(_ noteId: Int32, _ text: String) async throws {
        return try await Runtime.runOnce(DeUnit.self) { reax_note_update_note($0, noteId, text) }
    }

    static func deleteNote(_ noteId: Int32) async throws {
        return try await Runtime.runOnce(DeUnit.self) { reax_note_delete_note($0, noteId) }
    }
}
