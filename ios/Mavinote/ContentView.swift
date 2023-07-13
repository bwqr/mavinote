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
    @State private var notificationTasks: [Task<(), Never>] = []

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
                    if case .failure(let e) = await NoteViewModel.sync() {
                        appState.handleError(e)
                    }
                })

                tasks.append(Task {
                    for await result in AccountViewModel.accounts() {
                        switch result {
                        case .success(let accounts):
                            print("Number of accounts \(accounts.count)")
                            print("Number of notification tasks \(notificationTasks.count)")
                            notificationTasks.forEach { $0.cancel() }

                            notificationTasks = accounts
                                .filter { $0.kind == .Mavinote }
                                .map { account in
                                    Task {
                                        for await result in AccountViewModel.listenNotifications(account.id) {
                                            if case .failure(let e) = result {
                                                appState.handleError(e)
                                            }
                                        }
                                    }
                                }

                        case .failure(let e): appState.handleError(e)
                        }
                    }
                })
            }
            .onDisappear {
                tasks.forEach { $0.cancel() }
                notificationTasks.forEach { $0.cancel() }
            }
    }}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
