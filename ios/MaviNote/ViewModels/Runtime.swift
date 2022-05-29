import Foundation
import Serde

typealias OnNext = (_ deserializer: Deserializer) throws -> ()
typealias OnError = (_ error: ReaxError) -> ()
typealias OnComplete = () -> ()
typealias OnStart = (_ id: Int32) -> UnsafeMutableRawPointer

class Stream {
    private let onNext: OnNext
    private let onError: OnError
    private let onComplete: OnComplete
    private let onStart: OnStart
    private var _joinHandle: UnsafeMutableRawPointer?
    
    var joinHandle: UnsafeMutableRawPointer? {
        return _joinHandle
    }

    init(
        onNext: @escaping OnNext,
        onError: @escaping OnError,
        onComplete: @escaping OnComplete,
        onStart: @escaping OnStart
    ) {
        self.onNext = onNext
        self.onError = onError
        self.onComplete = onComplete
        self.onStart = onStart
    }
    
    func handle(_ bytes: [UInt8]) {
        let deserializer = BincodeDeserializer(input: bytes)

        do {
            switch try deserializer.deserialize_variant_index() {
            case 0: try self.onNext(deserializer)
            case 1: self.onError(try ReaxError.deserialize(deserializer))
            case 2: self.onComplete()
            default: fatalError("Unhandled variant")
            }
        } catch {
            print("failed to handle once", error)
        }

    }

    func start(_ streamId: Int32) {
        self._joinHandle = self.onStart(streamId)
    }
}

class Once {
    private let onNext: OnNext
    private let onError: OnError
    private let onStart: OnStart
    private var _joinHandle: UnsafeMutableRawPointer?

    var joinHandle: UnsafeMutableRawPointer? {
        return _joinHandle
    }

    init(
        onNext: @escaping OnNext,
        onError: @escaping OnError,
        onStart: @escaping OnStart
    ) {
        self.onNext = onNext
        self.onError = onError
        self.onStart = onStart
    }

    func handle(_ bytes: [UInt8]) {
        let deserializer = BincodeDeserializer(input: bytes)

        do {
            switch try deserializer.deserialize_variant_index() {
            case 0: try self.onNext(deserializer)
            case 1: self.onError(try ReaxError.deserialize(deserializer))
            default: fatalError("Unhandled variant")
            }
        } catch {
            print("failed to handle once", error)
        }
    }

    func start(_ onceId: Int32) {
        self._joinHandle = self.onStart(onceId)
    }
}

class Runtime {
    private static var _instance: Runtime?
    
    private var streams: [Int32 : Stream] = [:]
    private var onces: [Int32 : Once] = [:]
    
    static func initialize(storageDir: String) {
        if (_instance != nil) {
            return
        }

        _instance = Runtime(storageDir)
    }
    
    static func instance() -> Runtime {
        _instance!
    }
    
    private init(_ storageDir: String) {
       Thread
            .init(target: self, selector: #selector(initHandler), object: nil)
            .start()
 
        reax_init("http://192.168.1.50:8050/api", storageDir)
    }
   
    @objc private func initHandler() {
        let this = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())

        reax_init_handler(this) { id, isStream, bytes, bytesLen, ptr in
            let this = Unmanaged<AnyObject>.fromOpaque(ptr!).takeUnretainedValue() as! Runtime

            var bytesArray: [UInt8] = Array(repeating: 0, count: Int(bytesLen))

            for i in 0..<Int(bytesLen) {
                bytesArray[i] = bytes!.advanced(by: i).pointee
            }

            if isStream, let stream = this.streams[id] {
                stream.handle(bytesArray)
            } else if let once = this.onces[id] {
                once.handle(bytesArray)
            }
        }
        
        let _ = Unmanaged<AnyObject>.fromOpaque(this).takeRetainedValue()
    }

    func startOnce(_ once: Once) -> Int32 {
        var onceId = Int32.random(in: 0...Int32.max)

        while onces[onceId] != nil {
            onceId = Int32.random(in: 0...Int32.max)
        }

        onces[onceId] = once

        once.start(onceId)

        return onceId
    }

    func startStream(_ stream: Stream) -> Int32 {
        var streamId = Int32.random(in: 0...Int32.max)

        while streams[streamId] != nil {
            streamId = Int32.random(in: 0...Int32.max)
        }

        streams[streamId] = stream

        stream.start(streamId)

        return streamId
    }

    func abortStream(_ streamId: Int32) {
        if let stream = self.streams[streamId], let joinHandle = stream.joinHandle {
            reax_abort(joinHandle)
        }
    }

    func abortOnce(_ onceId: Int32) {
         if let once = self.onces[onceId], let joinHandle = once.joinHandle {
            reax_abort(joinHandle)
        }
    }
}
