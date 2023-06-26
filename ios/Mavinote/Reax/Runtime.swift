import Foundation
import Serde

protocol Future {
    // Return value indicates whether the future is completed or not
    func handle(_ bytes: [UInt8]) -> Bool

    func abort()
}

class Stream<T: DeserializeInto, E>: Future where E: Error, E: Deserialize {
    private let joinHandle: UnsafeMutableRawPointer
    private let continuation: AsyncStream<Result<T.Target, E>>.Continuation

    init(continuation: AsyncStream<Result<T.Target, E>>.Continuation, joinHandle: UnsafeMutableRawPointer) {
        self.continuation = continuation
        self.joinHandle = joinHandle
    }

    func handle(_ bytes: [UInt8]) -> Bool {
        let deserializer = BincodeDeserializer(input: bytes)

        do {
            switch try deserializer.deserialize_variant_index() {
            case 0:
                continuation.yield(Result.success(try T.deserialize(deserializer)))
                return false
            case 1:
                continuation.yield(Result.failure(try E.deserialize(deserializer)))
                return false
            case 2:
                continuation.finish()
                return true
            default: throw DeserializationError.invalidInput(issue: "Unknown variant index for Stream Message")
            }
        } catch {
            fatalError("failed to handle stream \(error)")
        }
    }

    func abort() {
        reax_abort(joinHandle)
    }
}

class Once<T: DeserializeInto, E>: Future where E: Error, E: Deserialize {
    let continuation: CheckedContinuation<Result<T.Target, E>, Never>

    init(continuation: CheckedContinuation<Result<T.Target, E>, Never>) {
        self.continuation = continuation
    }

    func handle(_ bytes: [UInt8]) -> Bool {
        let deserializer = BincodeDeserializer(input: bytes)

        do {
            switch try deserializer.deserialize_variant_index() {
            case 0: continuation.resume(returning: .success(try T.deserialize(deserializer)))
            case 1: continuation.resume(returning: .failure(try E.deserialize(deserializer)))
            default: throw DeserializationError.invalidInput(issue: "Unknown variant index for Once Message")
            }
        } catch {
            fatalError("failed to handle once \(error)")
        }

        return true
    }

    // Once is not abortable
    func abort() { }
}

class Runtime {
    private static var _instance: Runtime?

    private var futures: [Int32: any Future] = [:]

    static func initialize(storageDir: String) {
        if (_instance != nil) {
            return
        }

        _instance = Runtime(storageDir)
    }

    static func runStream<T: DeserializeInto>(_ type: T.Type, _ onStart: (_ id: Int32) -> UnsafeMutableRawPointer) -> AsyncStream<Result<T.Target, NoteError>> {
        Self.instance().runStream(T.self, onStart)
    }

    static func runStream<T: Deserialize>(_ onStart: (_ id: Int32) -> UnsafeMutableRawPointer) -> AsyncStream<Result<T, NoteError>> {
        return Self.instance().runStream(T.self, onStart)
    }

    static func runOnce<T: DeserializeInto>(_ _type: T.Type, _ onStart: (_ id: Int32) -> ()) async -> Result<T.Target, NoteError> {
        return await Self.instance().runOnce(T.self, onStart)
    }

    static func runOnce<T: Deserialize>(_ onStart: (_ id: Int32) -> ()) async -> Result<T, NoteError> {
        return await Self.instance().runOnce(T.self, onStart)
    }

    private static func instance() -> Runtime {
        _instance!
    }

    private init(_ storageDir: String) {
       Thread
            .init(target: self, selector: #selector(initHandler), object: nil)
            .start()

        reax_init(API_URL, WS_URL, storageDir)
    }

    private func runStream<T: DeserializeInto>(_ type: T.Type, _ onStart: (_ id: Int32) -> UnsafeMutableRawPointer) -> AsyncStream<Result<T.Target, NoteError>> {
        return AsyncStream { continuation in
            let id = generateId()

            let joinHandle = onStart(id)

            futures[id] = Stream<T, NoteError>(continuation: continuation, joinHandle: joinHandle)

            continuation.onTermination = { @Sendable _ in
                self.abort(id)
            }
        }
    }

    private func runOnce<T: DeserializeInto>(_ _type: T.Type, _ onStart: (_ id: Int32) -> ()) async -> Result<T.Target, NoteError> {
        return await withCheckedContinuation { continuation in
            let id = generateId()

            onStart(id)

            futures[id] = Once<T, NoteError>(continuation: continuation)
        }
    }

    private func generateId() -> Int32 {
        var id = Int32.random(in: 0...Int32.max)

        while futures[id] != nil {
            id = Int32.random(in: 0...Int32.max)
        }

        return id
    }

    private func abort(_ id: Int32) {
        guard let future = self.futures.removeValue(forKey: id) else {
            fatalError("Aborting an unknown future \(id)")
        }

        future.abort()
    }

    @objc private func initHandler() {
        let this = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())

        reax_init_handler(this) { id, isStream, bytes, bytesLen, ptr in
            let this = Unmanaged<AnyObject>.fromOpaque(ptr!).takeUnretainedValue() as! Runtime

            var bytesArray: [UInt8] = Array(repeating: 0, count: Int(bytesLen))

            for i in 0..<Int(bytesLen) {
                bytesArray[i] = bytes!.advanced(by: i).pointee
            }

            guard let future = this.futures[id] else {
                fatalError("A message with unknown id is received, \(id)")
            }

            if future.handle(bytesArray) {
                this.abort(id)
            }
        }

        let _ = Unmanaged<AnyObject>.fromOpaque(this).takeRetainedValue()
    }
}
