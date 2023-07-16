import SwiftUI

enum Route {
    case Accounts
    case Welcome
    case Folders
}

enum BusEvent {
    case ShowMessage(String)
}

class AppState : ObservableObject {
    @Published var activeRoute: Route? = .Folders
    private var eventContinuation: CheckedContinuation<BusEvent, Never>?

    func emit(_ event: BusEvent) {
        eventContinuation?.resume(returning: event)
    }

    func listenEvent() async -> BusEvent {
        return await withCheckedContinuation { continuation in
            self.eventContinuation = continuation
        }
    }

    func handleError(_ e: NoteError) {
        switch e {
        case .Mavinote(.NoConnection): emit(BusEvent.ShowMessage("No Internet Connection"))
        case .Mavinote(.DeviceDeleted(let accountId)): Task {
            switch await AccountViewModel.removeAccount(accountId) {
            case .failure(.Mavinote(.DeviceDeleted(_))):
                emit(.ShowMessage("Nested DeviceDeleted is encountered"))
                print("Nested DeviceDeleted is encountered")
            case .failure(let e): handleError(e)
            case .success(_): break
            }
        }
        default:
            emit(BusEvent.ShowMessage("\(e)"))
            debugPrint("Unhandled Error", e)
        }
    }

    func navigate(route: Route) {
        activeRoute = route
    }
}
