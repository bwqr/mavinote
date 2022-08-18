import SwiftUI

enum BackgroundFeaturesScreen {
    case Folders
}

struct BackgroundFeaturesView : View {
    @EnvironmentObject var appState: AppState
    @State var tasks: [Task<(), Error>] = []

    var body: some View {
        FoldersView()
        .onAppear {
            tasks.append(Task {
                while (true) {
                    switch await appState.listenEvent() {
                    case BusEvent.NoConnection: print("No connection")
                    }
                }
            })

            tasks.append(Task {
                do {
                    try await NoteViewModel().sync()
                } catch let error as ReaxError {
                    error.handle(appState)
                }
            })
        }
        .onDisappear {
            for task in tasks {
                task.cancel()
            }
        }
    }
}
