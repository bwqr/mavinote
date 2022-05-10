import SwiftUI

enum Screen {
    case Login
    case Folders
}

class AppState : ObservableObject {
    @Published var activeScreen: Screen? = nil

    func navigate(_ screen: Screen) {
        activeScreen = screen
    }
}

struct ContentView: View {
    @StateObject var appState = AppState()

    var body: some View {
        NavigationView {
            VStack {
                NavigationLink(destination: LoginView(), tag: Screen.Login, selection: $appState.activeScreen) {
                   EmptyView()
                }

                NavigationLink(destination: FoldersView(), tag: Screen.Folders, selection: $appState.activeScreen) {
                    EmptyView()
                }
            }
        }
        .environmentObject(appState)
        .onAppear {
            appState.activeScreen = Screen.Folders
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
