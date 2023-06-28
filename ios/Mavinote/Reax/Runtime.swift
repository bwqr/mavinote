import Foundation
import Serde

typealias OnStart = (_ id: Int32) -> UnsafeMutableRawPointer

protocol Future {
    // Return value indicates whether the future is completed or not
    func handle(_ bytes: [UInt8]) -> Bool

    func abort()
}

class Stream<T: Deserialize, E: Deserialize>: Future where E: Error {
    private let continuation: AsyncStream<Result<T, E>>.Continuation
    private var joinHandle: UnsafeMutableRawPointer?

    init(continuation: AsyncStream<Result<T, E>>.Continuation) {
        self.continuation = continuation
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
        guard let joinHandle = joinHandle else {
            fatalError("A Stream without a joinHandle is being aborted")
        }

        reax_abort(joinHandle)
    }

    func setJoinHandle(handle: UnsafeMutableRawPointer) {
        joinHandle = handle
    }
}

class Once<T: Deserialize, E: Deserialize>: Future where E: Error {
    private var continuation: CheckedContinuation<Result<T, E>, Never>?
    private var joinHandle: UnsafeMutableRawPointer?

    func handle(_ bytes: [UInt8]) -> Bool {
        guard let continuation = continuation else {
            fatalError("A Once without a continuation is being handled")
        }

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


    func abort() {
        guard let joinHandle = joinHandle else {
            fatalError("A Once without a joinHandle is being aborted")
        }

        reax_abort(joinHandle)
    }

    func setJoinHandle(handle: UnsafeMutableRawPointer) {
        joinHandle = handle
    }

    func setContinuation(cont: CheckedContinuation<Result<T, E>, Never>) {
        continuation = cont
    }
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

    static func runStream<T: Deserialize>(_ onStart: OnStart) -> AsyncStream<Result<T, NoteError>> {
        return Self.instance().runStream(onStart)
    }

    static func runOnceUnit(_ onStart: OnStart) async -> Result<(), NoteError> {
        let res: Result<UnitDeserialize, NoteError> = await Self.runOnce(onStart)

        return res.map { _ in }
    }

    static func runOnce<T: Deserialize>(_ onStart: OnStart) async -> Result<T, NoteError> {
        return await Self.instance().runOnce(onStart)
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

    private func runStream<T: Deserialize>(_ onStart: OnStart) -> AsyncStream<Result<T, NoteError>> {
        return AsyncStream { continuation in
            let id = generateId()
            let stream = Stream<T, NoteError>(continuation: continuation)
            futures[id] = stream

            stream.setJoinHandle(handle: onStart(id))


            continuation.onTermination = { @Sendable _ in
                self.abort(id)
            }
        }
    }

    private func runOnce<T: Deserialize>(_ onStart: OnStart) async -> Result<T, NoteError> {
        let id = generateId()
        let once = Once<T, NoteError>()
        futures[id] = once

        return await withTaskCancellationHandler(
            operation: {
                do {
                    try Task.checkCancellation()
                } catch {
                    return .failure(.TaskCancellation)
                }

                return await withCheckedContinuation { cont in
                    once.setContinuation(cont: cont)
                    once.setJoinHandle(handle: onStart(id))
                }
            },
            onCancel: { abort(id) }
        )
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

        reax_init_handler(this) { id, bytes, bytesLen, ptr in
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
