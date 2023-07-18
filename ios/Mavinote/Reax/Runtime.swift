import Foundation
import Serde

typealias AsyncStart = (_ id: Int32) -> UnsafeMutableRawPointer
typealias SyncStart = (_ deserializer: @convention(c) (UnsafePointer<UInt8>?, UInt) -> UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?

protocol Future {
    // Return value indicates whether the future is completed or not
    func handle(_ bytes: [UInt8]) -> Bool

    func abort()
}

class Stream<T: Deserialize>: Future {
    private let continuation: AsyncStream<T>.Continuation
    private var joinHandle: UnsafeMutableRawPointer?

    init(continuation: AsyncStream<T>.Continuation) {
        self.continuation = continuation
    }

    func handle(_ bytes: [UInt8]) -> Bool {
        let deserializer = BincodeDeserializer(input: bytes)

        do {
            switch try deserializer.deserialize_variant_index() {
            case 0:
                continuation.yield(try T.deserialize(deserializer))
                return false
            case 1:
                continuation.finish()
                return true
            default: throw DeserializationError.invalidInput(issue: "Unknown variant index for Stream Message")
            }
        } catch {
            fatalError("failed to deserialize in Stream \(error)")
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

class Once<T: Deserialize>: Future {
    private var continuation: CheckedContinuation<T, Never>?
    private var joinHandle: UnsafeMutableRawPointer?

    func handle(_ bytes: [UInt8]) -> Bool {
        guard let continuation = continuation else {
            fatalError("A Once without a continuation is being handled")
        }

        let deserializer = BincodeDeserializer(input: bytes)

        do {
            continuation.resume(returning: try T.deserialize(deserializer))
        } catch {
            fatalError("Failed to deserialize in Once \(error)")
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

    func setContinuation(cont: CheckedContinuation<T, Never>) {
        continuation = cont
    }
}

private class CriticalSection<T> {
    private var instance: T
    private let semaphore = DispatchSemaphore(value: 1)

    init(instance: T) {
        self.instance = instance
    }

    func enter<R>(_ callback: (inout T) -> R) -> R {
        semaphore.wait()
        let res = callback(&instance)
        semaphore.signal()
        return res
    }
}

class Runtime {
    private static var _instance: Runtime?

    private var futures: CriticalSection<[Int32: any Future]> = CriticalSection(instance: [:])

    static func initialize(storageDir: String) -> Result<(), String> {
        if (_instance != nil) {
            return .success(())
        }

        let result: Result<DeUnit, String> = run { reax_init(API_URL, WS_URL, storageDir, $0) }

        if case .failure(let failure) = result {
            return .failure(failure)
        }

        _instance = Runtime()

        Thread
            .init(target: _instance!, selector: #selector(initHandler), object: nil)
            .start()

        return .success(())
    }

    static func run<T: Deserialize, E: Deserialize>(_ onStart: SyncStart) -> Result<T, E> {
        let ptr = onStart { bytes, byteLen in
            let array = Array(UnsafeBufferPointer(start: bytes, count: Int(byteLen)))

            return UnsafeMutableRawPointer(Unmanaged.passRetained(BincodeDeserializer(input: array)).toOpaque())
        }

        let deserializer = Unmanaged<BincodeDeserializer>.fromOpaque(ptr!).takeRetainedValue()

        do {
            let index = try deserializer.deserialize_variant_index()

            switch index {
            case 0: return .success(try T.deserialize(deserializer))
            case 1: return .failure(try E.deserialize(deserializer))
            default: throw DeserializationError.invalidInput(issue: "Unknown variant for Result \(index)")
            }
        } catch {
            fatalError("Failed to deserialize sync function \(error)")
        }
    }

    static func runStream<T: Deserialize>(_ onStart: AsyncStart) -> AsyncStream<Result<T, NoteError>> {
        return Self.instance().runStream(onStart)
    }

    static func runOnceUnit(_ onStart: AsyncStart) async -> Result<(), NoteError> {
        let res: Result<DeUnit, NoteError> = await Self.runOnce(onStart)

        return res.map { _ in }
    }

    static func runOnce<T: Deserialize>(_ onStart: AsyncStart) async -> Result<T, NoteError> {
        return await Self.instance().runOnce(onStart)
    }

    private static func instance() -> Runtime {
        _instance!
    }

    private func runStream<T: Deserialize>(_ onStart: AsyncStart) -> AsyncStream<T> {
        return AsyncStream { continuation in
            let stream = Stream<T>(continuation: continuation)
            let id = insertFuture(future: stream)

            stream.setJoinHandle(handle: onStart(id))

            continuation.onTermination = { @Sendable _ in
                self.abort(id)
            }
        }
    }

    private func runOnce<T: Deserialize>(_ onStart: AsyncStart) async -> T {
        let once = Once<T>()
        let id = insertFuture(future: once)

        return await withTaskCancellationHandler(
            operation: {
                return await withCheckedContinuation { cont in
                    once.setContinuation(cont: cont)
                    once.setJoinHandle(handle: onStart(id))
                }
            },
            onCancel: { abort(id) }
        )
    }

    private func insertFuture(future: any Future) -> Int32 {
        return futures.enter { futures in
            var id = Int32.random(in: 0...Int32.max)

            while futures[id] != nil {
                id = Int32.random(in: 0...Int32.max)
            }

            futures[id] = future

            return id
        }
    }

    private func abort(_ id: Int32) {
        guard let future = futures.enter({ futures in futures.removeValue(forKey: id) }) else {
            return
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

            let future = this.futures.enter { futures in
                guard let future = futures[id] else {
                    fatalError("A message with unknown id is received, \(id)")
                }

                return future
            }

            if future.handle(bytesArray) {
                this.abort(id)
            }
        }

        let _ = Unmanaged<AnyObject>.fromOpaque(this).takeRetainedValue()
    }
}
