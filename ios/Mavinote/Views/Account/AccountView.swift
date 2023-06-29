import SwiftUI

struct AccountView : View {
    let accountName: String
    let accountId: Int32

    @EnvironmentObject var appState: AppState

    @State var account: Account?
    @State var mavinote: Mavinote?

    var body: some View {
        ZStack {
            if let account = account, let mavinote = mavinote {
                _AccountView(
                    account: account,
                    mavinote: mavinote
                )
            }
        }
        .navigationTitle(accountName)
        .onAppear {
            Task {
                switch await AccountViewModel.account(accountId) {
                case .success(let a):
                    account = a
                    if a?.kind == .Mavinote {
                        switch await AccountViewModel.mavinoteAccount(accountId) {
                        case .success(let m): mavinote = m
                        case .failure(let e): appState.handleError(e)
                        }
                    }
                case .failure(let e): appState.handleError(e)
                }
            }
        }
    }
}

private struct _AccountView : View {
    let account: Account
    let mavinote: Mavinote?

    @EnvironmentObject var appState: AppState
    @State var error: String?
    @State var showRemoveAccount = false
    @State var inProgress = false

    var body: some View {
        VStack(spacing: 12) {
            List {
                HStack {
                    Text("Name")
                    Spacer()
                    Text(account.name)
                        .foregroundColor(.gray)
                }
                .padding(.vertical)

                HStack {
                    Text("Kind")
                    Spacer()
                    Text(account.kind.rawValue.capitalized)
                        .foregroundColor(.gray)
                }
                .padding(.vertical)

                if let mavinote = mavinote {
                    HStack {
                        Text("Email")
                        Spacer()
                        Text(mavinote.email)
                            .foregroundColor(.gray)
                    }
                    .padding(.vertical)
                }
            }
            .listStyle(.plain)

            if mavinote != nil {
                List {
                    NavigationLink(destination: DevicesView(accountId: account.id)) {
                        Text("Devices")
                            .padding(.vertical)
                    }
                    
                    Button("Remove Account From Device") {
                        showRemoveAccount = true
                    }
                    .disabled(inProgress)
                    .foregroundColor(inProgress ? .gray : .red)
                    .padding(.vertical)
                    
                    NavigationLink(destination: AccountCloseView(accountId: account.id)) {
                        Text("Close Account")
                            .padding(.vertical)
                    }
                    .foregroundColor(.red)
                }
                .listStyle(.plain)
            }
        }
        .alert(
            "Are you sure about removing the account?",
            isPresented: $showRemoveAccount,
            actions: {
                Button("Remove", role: .destructive) {
                    if inProgress {
                        return
                    }

                    inProgress = true

                    Task {
                        switch await AccountViewModel.removeAccount(account.id) {
                        case .success(_):
                            appState.emit(.ShowMessage("Account is removed"))
                            appState.navigate(route: .Accounts)
                        case .failure(.Mavinote(.Message("cannot_delete_only_remaining_device"))):
                            error = "This device is the only remaining device for this account. If you want to close the account, choose Close Account option."
                        case .failure(let e): appState.handleError(e)
                        }

                        showRemoveAccount = false
                        inProgress = false
                    }
                }
                .disabled(inProgress)
                .foregroundColor(inProgress ? .gray : .red)
            },
            message: {
                Text("Removing account will only remove it from this device. Are you sure about removing the account from this device?")
            }
        )
        .alert(item: $error) { error in
            Alert(
                title: Text(""),
                message: Text(error),
                dismissButton: .default(Text("Ok"))
            )
        }
    }
}

struct AccountView_Preview : PreviewProvider {
    static var previews: some View {
        let account = Account(id: 1, name: "My Local Account", kind: .Local)
        let mavinote = Mavinote(email: "email@email.com", token: "mavinote account token")

        NavigationView {
            _AccountView(
                account: account,
                mavinote: mavinote
            )
            .navigationTitle(account.name)
        }
    }
}
