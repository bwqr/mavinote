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
    case RequireAuthorization(AccountId)
    case NoConnection

    struct AccountId : Identifiable {
        let id: Int32
    }
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

struct ContentView: View {
    @StateObject private var appState = AppState()
    @State var tasks: [Task<(), Never>] = []
    @State var accountToAuthorize: BusEvent.AccountId?

    var body: some View {
        FoldersView()
            .sheet(item: $accountToAuthorize) { accountId in
                AccountAuthorizeView(accountId: accountId.id)
            }
            .environmentObject(appState)
            .onAppear {
                tasks.append(Task {
                    while (true) {
                        switch await appState.listenEvent() {
                        case BusEvent.NoConnection: print("No connection")
                        case BusEvent.RequireAuthorization(let accountId): accountToAuthorize = accountId
                        }
                    }
                })

                tasks.append(Task {
                    do {
                        try await NoteViewModel.sync()
                    } catch let error as ReaxError {
                        error.handle(appState)
                    } catch {
                        fatalError("\(error)")
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
