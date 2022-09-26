import SwiftUI

struct AccountAuthorizeView : View {
    let accountId: Int32

    @State var tasks: [Task<(), Never>] = []
    @State var account: (Account, Mavinote)?
    @State var error: String?
    @State var inProgress = false

    @EnvironmentObject var appState: AppState
    @Environment(\.dismiss) var dismiss: DismissAction

    var body: some View {
        SafeContainer(value: $account) { account in
            _AccountAuthorizeView(
                account: account,
                error: $error,
                onAuthorize: { password in
                    if inProgress {
                        return
                    }

                    if password.isEmpty {
                        error = "Please type your password"
                        return
                    }

                    error = nil
                    inProgress = true

                    tasks.append(Task {
                        do {
                            try await NoteViewModel.authorizeAccount(accountId, password: password)
                            dismiss()
                        } catch let e as ReaxError {
                            switch e {
                            case .Http(.Unauthorized): error = "Wrong password, please try again"
                            case .Message(let message): error = message
                            default: e.handle(appState)
                            }
                        } catch {
                            fatalError("\(error)")
                        }

                        inProgress = false
                    })
                }
            )
        }
        .onAppear {
            tasks.append(Task {
                do {
                    if let a = try await NoteViewModel.account(accountId), let m = try await NoteViewModel.mavinoteAccount(accountId) {
                        account = (a, m)
                    }
                } catch let e as ReaxError {
                    e.handle(appState)
                }catch {
                    fatalError("\(error)")
                }
            })
        }
        .onDisappear {
            tasks.forEach { $0.cancel() }
        }
    }
}


private struct _AccountAuthorizeView : View {
    @Binding var account: (Account, Mavinote)
    @Binding var error: String?
    @State var password: String = ""

    let onAuthorize: (_ password: String) -> ()

    var body: some View {
        VStack(alignment: .leading) {
            Text("\(account.0.name) account with \(account.1.email) needs authorization")

            Text("Account Password")
                .font(.callout)
                .padding(.bottom, 8)
                .padding(.top, 16)

            SecureField("Password", text: $password)
                .padding(10)
                .background(InputBackground)
                .cornerRadius(5)
                .padding(.bottom, 16)

            Spacer()

            if let error = error {
                Text(error)
                    .foregroundColor(.red)
            }

            Button(action: {
                onAuthorize(password)
            }) {
                Text("Authorize")
                    .frame(maxWidth: .infinity)
                    .foregroundColor(.white)
            }
            .frame(maxWidth: .infinity)
            .padding(12)
            .background(.blue)
            .cornerRadius(8)
            .padding(.bottom, 12)

        }
        .padding(18)
    }
}

struct AccountAuthorizeView_Preview : PreviewProvider {
    static var previews: some View {
        let account = Account(id: 1, name: "My Account", kind: .Mavinote)
        let mavinote = Mavinote(email: "email@email.com", token: "token")
        let error = "Please type your password"

        VStack {
        }
        .sheet(isPresented: .constant(true)) {
            _AccountAuthorizeView(
                account: .constant((account, mavinote)),
                error: .constant(error),
                onAuthorize: { _ in }
            )
        }
    }
}
