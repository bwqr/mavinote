import SwiftUI

@main
struct MavinoteApp: App {
    let error: String?

    init() {
        do {
            let appSupportDir = try FileManager.default.url(for: .applicationSupportDirectory, in: .userDomainMask, appropriateFor: nil, create: true)

            switch Runtime.initialize(storageDir: appSupportDir.path) {
            case .success(_):
                NoteViewModel.initialize()
                self.error = nil
            case .failure(let error):
                self.error = error
            }
        } catch {
            print(error)
            fatalError("Unable to create application support directory")
        }
    }

    var body: some Scene {
        WindowGroup {
            if let error = error {
                UnrecoverableErrorView(error: error)
            } else {
                ContentView()
            }
        }
    }
}

private struct UnrecoverableErrorView: View {
    let error: String

    var body: some View {
        VStack(alignment: .leading, spacing: 24.0) {
            Text("An unrecoverable error is encountered while initializing the application")

            Text("Error: \(error)")
        }
    }
}
