import SwiftUI

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

enum BusEvent {
    case ShowMessage(String)
}

class AppState : ObservableObject {
    private var eventContinuation: CheckedContinuation<BusEvent, Never>?

    func emit(_ event: BusEvent) {
        eventContinuation?.resume(returning: event)
    }

    func listenEvent() async -> BusEvent {
        return await withCheckedContinuation { continuation in
            self.eventContinuation = continuation
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
    @State private var toast: Toast?

    var body: some View {
        FoldersView()
            .sheet(item: $toast) { toast in
                Text(toast.message)
            }
            .environmentObject(appState)
            .onAppear {
                tasks.append(Task {
                    while (true) {
                        switch await appState.listenEvent() {
                        case BusEvent.ShowMessage(let message): toast = Toast(message: message)
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
