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

enum Screen {
    case Login
    case BackgroundFeatures
}

enum BusEvent {
    case NoConnection
}

class AppState : ObservableObject {
    private var eventContinuation: CheckedContinuation<BusEvent, Never>?
    @Published var activeScreen: Screen?

    func navigate(_ screen: Screen) {
        activeScreen = screen
    }

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
