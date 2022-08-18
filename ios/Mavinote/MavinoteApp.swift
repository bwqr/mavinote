import SwiftUI

@main
struct MavinoteApp: App {
    init() {
        do {
            let appSupportDir = try FileManager.default.url(for: .applicationSupportDirectory, in: .userDomainMask, appropriateFor: nil, create: true)

            Runtime.initialize(storageDir: appSupportDir.path)
        } catch {
            print(error)
            fatalError("Unable to create application support directory")
        }
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
