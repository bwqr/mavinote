import SwiftUI
import AlertToast

struct SafeContainer<T, Content: View> : View {
    @Binding var value: T?

    @ViewBuilder var content: (_ value: Binding<T>) -> Content

    var body: some View {
        if value != nil {
            content(Binding($value)!)
        } else {
            ZStack { }
        }
    }
}

private struct Toast: Identifiable {
    var id: String { get { message } }

    let message: String
}

struct ContentView: View {
    @StateObject private var appState = AppState()
    @State private var tasks: [Task<(), Never>] = []
    @State private var showToast = false
    @State private var toast: String = ""

    var body: some View {
        FoldersView()
            .toast(isPresenting: $showToast, duration: 2.0) {
                AlertToast(displayMode: .hud, type: .regular, title: toast)
            }
            .environmentObject(appState)
            .onAppear {
                tasks.append(Task {
                    while (true) {
                        switch await appState.listenEvent() {
                        case BusEvent.ShowMessage(let message):
                            showToast = true
                            toast = message
                        }
                    }

                    fatalError("Bus listening is stopped")
                })

                tasks.append(Task {
                    switch await NoteViewModel.sync() {
                    case .failure(let e): e.handle(appState)
                    default: break
                    }
                })
            }
            .onDisappear {
                tasks.forEach { $0.cancel() }
            }
    }}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
