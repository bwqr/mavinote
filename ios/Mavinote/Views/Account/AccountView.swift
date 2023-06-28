import SwiftUI

struct AccountView : View {
    let accountName: String
    let accountId: Int32

    @EnvironmentObject var appState: AppState
    @Environment(\.dismiss) var dismiss: DismissAction

    @State var tasks: [Task<(), Never>] = []
    @State var account: Account?
    @State var mavinote: Mavinote?

    @State var inProgress = false

    var body: some View {
        SafeContainer(value: $account) { account in
            _AccountView(
                account: account,
                mavinote: $mavinote,
                onDelete: {
                    if (inProgress) {
                        return
                    }

                    inProgress = true

                    tasks.append(Task {
                        switch await AccountViewModel.removeAccount(accountId) {
                        case .success(_):
                            dismiss()
                            appState.emit(.ShowMessage("Account is deleted successfully"))
                        case .failure(let e): appState.handleError(e)
                        }

                        inProgress = false
                    })
                }
            )
        }
        .navigationTitle(accountName)
        .onAppear {
            tasks.append(Task {
                switch await AccountViewModel.account(accountId) {
                case .success(let a): account = a
                case .failure(let e): e.handle(appState)
                }
            })
        }
        .onDisappear {
            tasks.forEach { $0.cancel() }
        }
    }
}

private struct _AccountView : View {
    @Binding var account: Account
    @Binding var mavinote: Mavinote?

    let onDelete: () -> ()

    @State var showEdit = false

    var body: some View {
        VStack(spacing: 12) {
            List {
                HStack {
                    Text("Name")
                    Spacer()
                    Text(account.name)
                        .foregroundColor(.gray)
                }

                HStack {
                    Text("Kind")
                    Spacer()
                    Text(account.kind.rawValue.capitalized)
                        .foregroundColor(.gray)
                }

                if let mavinote = mavinote {
                    HStack {
                        Text("Email")
                        Spacer()
                        Text(mavinote.email)
                            .foregroundColor(.gray)
                    }
                }
            }
        }
        .toolbar {
            Button("Edit") {
                showEdit = true
            }
            .confirmationDialog("Edit Account", isPresented: $showEdit)  {
                Button("Delete", role: .destructive) {
                    onDelete()
                }
            }
        }
    }
}

struct AccountView_Preview : PreviewProvider {
    static var previews: some View {
        let account = Account(id: 1, name: "My Local Account", kind: .Local)
        let mavinote = Mavinote(email: "email@email.com", token: "mavinote account token")

        NavigationView {
            _AccountView(
                account: .constant(account),
                mavinote: .constant(mavinote),
                onDelete: { }
            )
            .navigationTitle(account.name)
        }
    }
}
