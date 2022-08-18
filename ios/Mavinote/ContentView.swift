import SwiftUI

enum Screen {
    case Login
    case BackgroundFeatures
}

enum BusEvent {
    case NoConnection
}

class AppState : ObservableObject {
    private var continuation: CheckedContinuation<BusEvent, Never>?
    @Published var activeScreen: Screen? = nil

    func navigate(_ screen: Screen) {
        activeScreen = screen
    }

    func emit(_ event: BusEvent) {
        continuation?.resume(returning: event)
    }

    func listenEvent() async -> BusEvent {
        return await withCheckedContinuation { continuation in
            self.continuation = continuation
        }
    }
}

struct ContentView: View {
    @StateObject private var appState = AppState()

    var body: some View {
        BackgroundFeaturesView()
            .environmentObject(appState)
            .onAppear {
                appState.activeScreen = Screen.BackgroundFeatures
            }
    }}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
