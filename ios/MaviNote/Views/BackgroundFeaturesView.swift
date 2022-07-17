import SwiftUI

enum BackgroundFeaturesScreen {
    case Folders
}

struct BackgroundFeaturesView : View {
    @EnvironmentObject var appState: AppState
    @State var tasks: [Task<(), Error>] = []
    @State var activeSyncs: Int32 = 0

    var body: some View {
        FoldersView()
        .navigationBarBackButtonHidden(true)
        .onAppear {
            tasks.append(Task {
                let stream = NoteViewModel().activeSyncs()

                for await result in stream {
                    switch result {
                    case .success(let s): activeSyncs = s
                    case .failure(let e): debugPrint("error in active syncs \(e)")
                    }
                }
            })

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
