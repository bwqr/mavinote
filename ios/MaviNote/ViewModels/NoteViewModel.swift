class NoteViewModel {
    func sync() async throws -> () {
        return try await withCheckedThrowingContinuation { continuation in
            Runtime.instance().startOnce(Once(
                onNext: { deserializer in continuation.resume(returning: ()) },
                onError: { continuation.resume(throwing: $0)},
                onStart: { reax_note_sync($0) }
            ))
        }
    }

    func activeSyncs() -> AsyncStream<Result<Int32, ReaxError>> {
        return AsyncStream { continuation in
             let streamId = Runtime.instance().startStream(Stream(
                onNext: { continuation.yield(Result.success(try $0.deserialize_i32())) },
                onError: { continuation.yield(Result.failure($0))},
                onComplete: { continuation.finish() },
                onStart: { reax_note_active_syncs($0)}
            ))

            continuation.onTermination = { @Sendable _ in
                Runtime.instance().abortStream(streamId)
            }
        }
    }

    func folders() -> AsyncStream<Result<[Folder], ReaxError>> {
        return AsyncStream { continuation in
             let streamId = Runtime.instance().startStream(Stream(
                onNext: { continuation.yield(Result.success(try deserializeList($0) { try Folder.deserialize($0) })) },
                onError: { continuation.yield(Result.failure($0))},
                onComplete: { continuation.finish() },
                onStart: { reax_note_folders($0)}
            ))

            continuation.onTermination = { @Sendable _ in
                Runtime.instance().abortStream(streamId)
            }
        }
    }

    func folder(_ folderId: Int32) async throws -> () {
        return try await withCheckedThrowingContinuation { continuation in
            Runtime.instance().startOnce(Once(
                onNext: { deserializer in continuation.resume(returning: ()) },
                onError: { continuation.resume(throwing: $0)},
                onStart: { reax_note_folder($0, folderId) }
            ))
        }
    }

    func createFolder(_ name: String) async throws -> () {
        return try await withCheckedThrowingContinuation { continuation in
            Runtime.instance().startOnce(Once(
                onNext: { deserializer in continuation.resume(returning: ()) },
                onError: { continuation.resume(throwing: $0)},
                onStart: { reax_note_create_folder($0, name) }
            ))
        }
    }

    func noteSummaries(_ folderId: Int32) -> AsyncStream<Result<[Note], ReaxError>> {
        return AsyncStream { continuation in
            let streamId = Runtime.instance().startStream(Stream(
                onNext: { continuation.yield(Result.success(try deserializeList($0) { try Note.deserialize($0) })) },
                onError: { continuation.yield(Result.failure($0)) },
                onComplete: { continuation.finish() },
                onStart: { reax_note_note_summaries($0, folderId) }
            ))

            continuation.onTermination = { @Sendable _ in
                Runtime.instance().abortStream(streamId)
            }
        }
    }

    func note(_ noteId: Int32) async throws -> Note? {
        return try await withCheckedThrowingContinuation { continuation in
            Runtime.instance().startOnce(Once(
                onNext: { continuation.resume(returning: try deserializeOption($0) {
                    try Note.deserialize($0)
                }) },
                onError: { continuation.resume(throwing: $0)},
                onStart: {reax_note_note($0, noteId)}
            ))
        }
    }

    func createNote(_ folderId: Int32) async throws -> Int32 {
        return try await withCheckedThrowingContinuation { continuation in
            Runtime.instance().startOnce(Once(
                onNext: { continuation.resume(returning: try $0.deserialize_i32()) },
                onError: { continuation.resume(throwing: $0)},
                onStart: { reax_note_create_note($0, folderId) }
            ))
        }
    }

    func updateNote(_ noteId: Int32, _ text: String) async throws -> () {
        return try await withCheckedThrowingContinuation { continuation in
            Runtime.instance().startOnce(Once(
                onNext: { deserializer in continuation.resume(returning: ()) },
                onError: { continuation.resume(throwing: $0)},
                onStart: { reax_note_update_note($0, noteId, text) }
            ))
        }
    }

    func deleteNote(_ noteId: Int32) async throws -> () {
        return try await withCheckedThrowingContinuation { continuation in
            Runtime.instance().startOnce(Once(
                onNext: { deserializer in continuation.resume(returning: ()) },
                onError: { continuation.resume(throwing: $0)},
                onStart: { reax_note_delete_note($0, noteId) }
            ))
        }
    }
}
