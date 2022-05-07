import Foundation
import Serde

protocol Resume {
    func handle(ok: Bool, bytes: [UInt8])
    
}

class AsyncWait<T> : Resume {
    let continuation: CheckedContinuation<T, Error>
    let deserialize: (_ deserializer: Deserializer) throws -> T
    init(_ continuation: CheckedContinuation<T, Error>, _ deserialize: @escaping (_ deserializer: Deserializer) throws -> T) {
        self.continuation = continuation
        self.deserialize = deserialize
    }
    
    func handle(ok: Bool, bytes: [UInt8]) {
        print("received ", ok, bytes.capacity)
        let deserializer = BincodeDeserializer.init(input: bytes)

        do {
            if ok {
                self.continuation.resume(with: Result.success(try self.deserialize(deserializer)))
            } else {
                let error = try ReaxError.deserialize(deserializer)
                self.continuation.resume(throwing: error)
            }
        } catch {
            print("failed to resume continuation", error)
        }
    }
}

class Runtime {
    private static var _instance: Runtime?
    
    private var waits: [Int32 : Resume] = [:]
    
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
 
        reax_init("http://192.168.1.3:8050/api", storageDir)
    }
   
    @objc private func initHandler() {
        let this = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())

        reax_init_handler(this) { waitId, ok, bytes, bytesLen, ptr in
            let this = Unmanaged<AnyObject>.fromOpaque(ptr!).takeUnretainedValue() as! Runtime

            guard let resume = this.waits[waitId] else {
                return
            }

            var bytesArray: [UInt8] = Array(repeating: 0, count: Int(bytesLen))

            for i in 0..<Int(bytesLen) {
                bytesArray[i] = bytes!.advanced(by: i).pointee
            }

            resume.handle(ok: ok, bytes: bytesArray)
        }
        
        let _ = Unmanaged<AnyObject>.fromOpaque(this).takeRetainedValue()
    }
 
    func wait(resume: Resume) -> Int32 {
        var waitId = Int32.random(in: 0...Int32.max)

        while waits[waitId] != nil {
            waitId = Int32.random(in: 0...Int32.max)
        }
        
        waits[waitId] = resume
        
        return waitId
    }
}
