import SwiftUI

struct NavigationsView: View {
    @EnvironmentObject var appState: AppState
    var body: some View {
        List {
            NavigationLink(
                destination: AccountsView(),
                tag: Route.Accounts,
                selection: $appState.activeRoute
            ) {
                Text("Accounts")
                    .padding(.vertical)
            }

            NavigationLink(destination: WelcomeView()) {
                Text("Welcome Page")
                    .padding(.vertical)
            }

            NavigationLink(destination: HelpView()) {
                Text("Help")
                    .padding(.vertical)
            }
        }
        .navigationTitle("Mavinote")
    }
}
