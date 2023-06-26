import SwiftUI

private class AccountAddController: ObservableObject {
    let onRequestVerification: () -> ()

    init(onRequestVerification: @escaping () -> Void) {
        self.onRequestVerification = onRequestVerification
    }
}

struct AccountAddView : View {
    @EnvironmentObject var appState: AppState
    @Environment(\.dismiss) var dismiss: DismissAction
    @StateObject private var controller = AccountAddController(onRequestVerification: { print("Requesting Verification") })

    @State var tasks: [Task<(), Never>] = []
    @State var inProgress = false
    @State var error: String?

    var body: some View {
        ChooseAccountAddKindView()
            .environmentObject(controller)
     }
}

private struct ChooseAccountAddKindView: View {
    @EnvironmentObject var controller: AccountAddController

    var body: some View {
        VStack(alignment: .leading, spacing: 32.0) {
            Text("You can create a new account or add an already existing account")

            Text("If you have already created an account from another device, you can also access it from this device")

            List {
                NavigationLink(destination: EnterAccountInfoView().environmentObject(controller)) {
                    Text("Add an Existing Account")
                        .padding(.vertical)
                }

                NavigationLink(destination: { Text("Hello") }) {
                    Text("Create a New Account")
                        .padding(.vertical)
                }
            }
            .listStyle(.plain)

            Spacer()
        }
        .padding([.horizontal, .bottom], 12)
        .navigationTitle("Add Account")
    }
}

private struct EnterAccountInfoView: View {
    @EnvironmentObject var controller: AccountAddController
    @State var email: String = ""

    var body: some View {
        VStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 32.0) {
                    Text("Email address is used to identify accounts.")

                    Text("Please enter the email address of the account you want to add.")

                    VStack(alignment: .leading) {
                        Text("Email")
                            .font(.callout)

                        TextField("Email", text: $email)
                            .padding(12)
                            .background(InputBackground)
                            .cornerRadius(8)
                    }
                }
                .padding([.horizontal, .bottom], 12)
            }

            Button(action: { controller.onRequestVerification() }) {
                Text("Request Verification")
                    .frame(maxWidth: .infinity)
                    .foregroundColor(.white)
            }
            .frame(maxWidth: .infinity)
            .padding(12)
            .background(.blue)
            .cornerRadius(8)
            .padding([.horizontal, .bottom], 12)
        }
    }
}

private struct AddExistingAccountView : View {
    let onAdd: (_ name: String, _ email: String, _ password: String, _ createAccount: Bool) -> ()
    @Binding var error: String?

    @State var name = ""
    @State var email = ""
    @State var password = ""
    @State var createAccount = false

    var body: some View {
        ScrollView {
            VStack(alignment: .leading) {
                Text("You can add your Mavinote account or create a new one if you do not have. Mavinote account lets you synchronize your notes with other devices")
                    .font(.footnote)
                    .padding(.bottom, 12)

                Text("Account Name")
                    .font(.callout)
                    .padding(.bottom, 8)

                TextField("Account Name", text: $name)
                    .padding(10)
                    .background(InputBackground)
                    .cornerRadius(5)
                    .padding(.bottom, 16)

                Text("Email")
                    .font(.callout)
                    .padding(.bottom, 8)

                TextField("Email", text: $email)
                    .textInputAutocapitalization(.never)
                    .textContentType(.emailAddress)
                    .keyboardType(.emailAddress)
                    .padding(10)
                    .background(InputBackground)
                    .cornerRadius(5)
                    .padding(.bottom, 16)

                Text("Password")
                    .font(.callout)
                    .padding(.bottom, 8)

                SecureField("Password", text: $password)
                    .textContentType(.password)
                    .padding(10)
                    .background(InputBackground)
                    .cornerRadius(5)
                    .padding(.bottom, 16)

                HStack {
                    Image(systemName: createAccount ? "checkmark.square.fill" : "square")
                        .onTapGesture {
                            createAccount = !createAccount
                        }

                    Text("I do not have a Mavinote account, create a new one")
                        .font(.subheadline)
                }

                if let error = error {
                    Text(error)
                        .foregroundColor(.red)
                }

                Button(action: {
                    onAdd(name, email, password, createAccount)
                }) {
                    Text("Add Account")
                        .frame(maxWidth: .infinity)
                        .foregroundColor(.white)
                }
                .frame(maxWidth: .infinity)
                .padding(12)
                .background(.blue)
                .cornerRadius(8)
                .padding(.bottom, 12)

            }
            .padding([.leading, .trailing], 18)
        }
    }
}

struct ChooseAccountAddKind_Preview: PreviewProvider {
    static var previews: some View {
        NavigationView {
            ChooseAccountAddKindView()
        }
    }
}

struct EnterAccountInfo_Preview: PreviewProvider {
    static var previews: some View {
        NavigationView {
            EnterAccountInfoView()
                .environmentObject(AccountAddController(onRequestVerification: { }))
        }
    }
}
