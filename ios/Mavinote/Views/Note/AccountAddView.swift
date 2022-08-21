import SwiftUI

struct AccountAddView : View {
    @Environment(\.dismiss) var dismiss: DismissAction

    @State var tasks: [Task<(), Never>] = []
    @State var inProgress = false
    @State var error: String?

    var body: some View {
        _AccountAddView(
            onAdd: { name, email, password, createAccount in
                if (inProgress) {
                    return
                }

                if name.isEmpty || email.isEmpty || password.isEmpty {
                    error = "Please fill the fields"
                    return
                }

                error = nil
                inProgress = true

                tasks.append(Task {
                    do {
                        try await NoteViewModel().addAccount(name, email, password, createAccount)
                        dismiss()
                    } catch {
                        print("failed to add an account")
                    }

                    inProgress = false
                })
            },
            error: $error
        )
    }
}

private struct _AccountAddView : View {
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
        .navigationTitle("Add Account")
    }
}

struct AccountAddView_Preview : PreviewProvider {
    static var previews: some View {
        let error: String? = nil

        NavigationView {
            _AccountAddView(
                onAdd: { _, _, _, _ in },
                error: .constant(error)
            )
        }
    }
}
