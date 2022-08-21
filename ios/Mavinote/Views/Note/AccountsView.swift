import SwiftUI

struct AccountsView : View {
    @EnvironmentObject var appState: AppState

    @State var tasks: [Task<(), Never>] = []
    @State var accounts: [Account] = []

    var body: some View {
        _AccountsView(accounts: $accounts)
            .onAppear {
                tasks.append(Task {
                    let stream = NoteViewModel().accounts()

                    for await result in stream {
                        switch result {
                        case .success(let a): accounts = a
                        case .failure(_): appState.navigate(Screen.Login)
                        }
                    }
                })
            }
    }
}

private struct _AccountsView : View {
    @Binding var accounts: [Account]

    var body: some View {
        VStack {
            List(accounts) { account in
                NavigationLink(
                    destination: AccountView(accountName: account.name, accountId: account.id)
                ) {
                    Text(account.name)
                }
            }

            Spacer()

            HStack {
                Spacer()
                NavigationLink(destination: AccountAddView()) {
                    Image(systemName: "person.crop.circle.badge.plus")
                        .padding(EdgeInsets(top: 2, leading: 12, bottom: 12, trailing: 24))
                        .foregroundColor(.blue)
                }
            }
        }
        .navigationTitle("Accounts")
    }
}

struct AccountsView_Preview : PreviewProvider {
    static var previews: some View {
        let accounts = [
            Account(id: 1, name: "Local", kind: .Local),
            Account(id: 2, name: "My Remote Account", kind: .Mavinote)
        ]

        NavigationView {
            _AccountsView(accounts: .constant(accounts))
        }
    }
}
