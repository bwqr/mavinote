import SwiftUI

enum Route {
    case Accounts
}

enum BusEvent {
    case ShowMessage(String)
}

class AppState : ObservableObject {
    @Published var activeRoute: Route?
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
        default:
            emit(BusEvent.ShowMessage("\(e)"))
            debugPrint("Unhandled Error", e)
        }
    }

    func navigate(route: Route) {
        activeRoute = route
    }
}
