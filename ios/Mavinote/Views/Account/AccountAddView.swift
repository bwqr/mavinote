import SwiftUI

private func onAccountAdd(appState: AppState) {
    appState.navigate(route: .Accounts)
    appState.emit(.ShowMessage("Account is successfully added"))
}

struct AccountAddView : View {
    @EnvironmentObject var appState: AppState

    @State var tasks: [Task<(), Never>] = []
    @State var inProgress = false
    @State var error: String?

    var body: some View {
        ChooseAccountAddKindView()
     }
}

private struct ChooseAccountAddKindView: View {
    @EnvironmentObject var appState: AppState

    var body: some View {
        VStack(alignment: .leading, spacing: 32.0) {
            Text("You can create a new account or add an already existing account")

            Text("If you have already created an account from another device, you can also access it from this device")

            List {
                NavigationLink(destination: EnterAccountInfoView()) {
                    Text("Add an Existing Account")
                        .padding(.vertical)
                }

                NavigationLink(destination: SendVerificationCodeView()) {
                    Text("Create a New Account")
                        .padding(.vertical)
                }
            }
            .listStyle(.plain)

            Spacer()
        }
        .padding(.all, 12)
        .navigationTitle("Add Account")
    }
}

private struct EnterAccountInfoView: View {
    enum ValidationErrors {
        case InvalidEmail
    }

    @EnvironmentObject var appState: AppState

    @State var email: String = ""
    @State var token: String = ""
    @State var showPublicKey = false
    @State var validationErrors = Set<ValidationErrors>()
    @State var error: String?
    @State var inProgress = false

    var body: some View {
        VStack {
            NavigationLink(
                isActive: $showPublicKey,
                destination: { ShowPublicKeyView(email: email, token: token) }
            ) {
                EmptyView()
            }

            ScrollView {
                VStack(alignment: .leading, spacing: 32.0) {
                    Text("Email address is used to identify accounts.")

                    Text("Please enter the email address of the account you want to add.")

                    VStack(alignment: .leading) {
                        Text("Email")
                            .font(.callout)

                        TextField("Email", text: $email)
                            .textInputAutocapitalization(.never)
                            .textContentType(.emailAddress)
                            .keyboardType(.emailAddress)
                            .textFieldStyle(.roundedBorder)
                            .cornerRadius(8)

                        if validationErrors.contains(.InvalidEmail) {
                            Text("Please specify a valid email")
                                .foregroundColor(.red)
                        }
                    }
                }
                .padding(.all, 12)
            }

            Button(action: {
                if inProgress {
                    return
                }

                validationErrors = Set()

                if email.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
                    validationErrors.insert(.InvalidEmail)
                }

                if !validationErrors.isEmpty {
                    return
                }

                inProgress = true

                Task {
                    switch await AccountViewModel.requestVerification(email) {
                    case .success(let t):
                        token = t
                        showPublicKey = true
                    case .failure(let e):
                        switch e {
                        case .Mavinote(.Message("email_not_found")):
                            error = "Email could not be found. Please check your input."
                        case .Mavinote(.Message("device_exists_but_passwords_mismatch")):
                            error =
                                "An unexpected state is occurred. A device with our public key is already added. " +
                                "However, the passwords do not match. In order to resolve the issue, from a device this account is already added, " +
                                "you can remove the device with our public key and try to add account again."
                        case .Mavinote(.Message("device_already_exists")):
                            switch await AccountViewModel.addAccount(email) {
                            case .success(_):
                                onAccountAdd(appState: appState)
                            case .failure(let e): appState.handleError(e)
                            }
                        case .Storage(.EmailAlreadyExists):
                            error = "An account with this email already exists. You can find it under Accounts page."
                        default: appState.handleError(e)
                        }
                    }

                    inProgress = false
                }
            }) {
                Text("Request Verification")
                    .frame(maxWidth: .infinity)
                    .foregroundColor(.white)
            }
            .frame(maxWidth: .infinity)
            .padding(12)
            .background(inProgress ? .gray : .blue)
            .disabled(inProgress)
            .cornerRadius(8)
            .padding(.all, 12)
        }
        .navigationTitle("Add an Existing Account")
        .alert(item: $error) { error in
            Alert(
                title: Text(""),
                message: Text(error),
                dismissButton: .default(Text("Ok"))
            )
        }
    }
}

private struct ShowPublicKeyView: View {
    let email: String
    let token: String
    @State var verificationTask: Task<(), Never>?

    @EnvironmentObject var appState: AppState
    @State var publicKey: String?

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 32.0) {
                Text("A verification request is sent to server for \(email) email address.")

                Text(
                    "In order to complete the progress, on the other device that has already account added, " +
                    "you need to choose Add Device and enter the Public Key displayed below. Please note that Public Key does not contain any line break."
                )

                Text("You have 5 min to complete progress")

                if let publicKey = publicKey {
                    VStack(alignment: .leading, spacing: 12.0) {
                        Text("Public Key:")

                        Text(publicKey)
                            .bold()
                    }
                }
            }
            .padding(.all, 12.0)
        }
        .navigationTitle("Enter Public Key")
        .onAppear {
            Task {
                switch await AccountViewModel.publicKey() {
                case .success(let p): publicKey = p
                case .failure(let e): appState.handleError(e)
                }
            }

            verificationTask = Task {
                switch await AccountViewModel.waitVerification(token) {
                case .success(_):
                    switch await AccountViewModel.addAccount(email) {
                    case .success(_):
                        onAccountAdd(appState: appState)
                    case .failure(let e): appState.handleError(e)
                    }
                case .failure(let e):
                    appState.handleError(e)
                }
            }
        }
        .onDisappear {
            verificationTask?.cancel()
        }
    }
}

private struct SendVerificationCodeView: View {
    enum ValidationErrors {
        case InvalidEmail
    }

    @EnvironmentObject var appState: AppState

    @State var email: String = ""
    @State var showVerifyCode = false
    @State var validationErrors = Set<ValidationErrors>()
    @State var error: String?
    @State var inProgress = false

    var body: some View {
        VStack {
            NavigationLink(
                isActive: $showVerifyCode,
                destination: { VerifyCodeView(email: email) }
            ) {
                EmptyView()
            }

            ScrollView {
                VStack(alignment: .leading, spacing: 32.0) {
                    Text("Email address is used to identify accounts.")

                    Text("Please enter an email address to create an account for it.")

                    VStack(alignment: .leading) {
                        Text("Email")
                            .font(.callout)

                        TextField("Email", text: $email)
                            .textInputAutocapitalization(.never)
                            .textContentType(.emailAddress)
                            .keyboardType(.emailAddress)
                            .textFieldStyle(.roundedBorder)
                            .cornerRadius(8)

                        if validationErrors.contains(.InvalidEmail) {
                            Text("Please specify a valid email")
                                .foregroundColor(.red)
                        }
                    }
                }
                .padding(.all, 12)
            }

            Button(action: {
                if inProgress {
                    return
                }

                validationErrors = Set()

                if email.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
                    validationErrors.insert(.InvalidEmail)
                }

                if !validationErrors.isEmpty {
                    return
                }

                inProgress = true

                Task {
                    switch await AccountViewModel.sendVerificationCode(email) {
                    case .success(_):
                        showVerifyCode = true
                    case .failure(let e):
                        switch e {
                        case .Mavinote(.Message("email_already_used")):
                            error = "This email address is already used for another account. You can add it by choosing Add an Existing Account option."
                        case .Storage(.EmailAlreadyExists):
                            error = "An account with this email already exists. You can find it under Accounts page."
                        default: appState.handleError(e)
                        }
                    }

                    inProgress = false
                }
            }) {
                Text("Send Verification Code")
                    .frame(maxWidth: .infinity)
                    .foregroundColor(.white)
            }
            .frame(maxWidth: .infinity)
            .padding(12)
            .background(inProgress ? .gray : .blue)
            .disabled(inProgress)
            .cornerRadius(8)
            .padding(.all, 12)
        }
        .navigationTitle("Create a New Account")
        .alert(item: $error) { error in
            Alert(
                title: Text(""),
                message: Text(error),
                dismissButton: .default(Text("Ok"))
            )
        }
    }
}

private struct VerifyCodeView: View {
    enum ValidationErrors {
        case InvalidCode
    }

    let email: String

    @EnvironmentObject var appState: AppState

    @State var code: String = ""
    @State var validationErrors = Set<ValidationErrors>()
    @State var error: String?
    @State var inProgress = false

    var body: some View {
        VStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 32.0) {
                    Text("An 8 digit verification code is sent to \(email) email address.")

                    Text("Please enter verification code to ensure that email belongs to you.")

                    VStack(alignment: .leading) {
                        Text("Code")
                            .font(.callout)

                        TextField("Code", text: $code)
                            .textInputAutocapitalization(.never)
                            .textContentType(.oneTimeCode)
                            .keyboardType(.numberPad)
                            .textFieldStyle(.roundedBorder)
                            .cornerRadius(8)

                        if validationErrors.contains(.InvalidCode) {
                            Text("Please specify the verification code")
                                .foregroundColor(.red)
                        }
                    }
                }
                .padding(.all, 12)
            }

            Button(action: {
                if inProgress {
                    return
                }

                validationErrors = Set()

                if code.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
                    validationErrors.insert(.InvalidCode)
                }

                if !validationErrors.isEmpty {
                    return
                }

                inProgress = true

                Task {
                    switch await AccountViewModel.signUp(email, code) {
                    case .success(_):
                        appState.emit(.ShowMessage("Account is successfully created"))
                        appState.navigate(route: .Accounts)
                    case .failure(let e):
                        switch e {
                        case .Storage(.EmailAlreadyExists):
                            error = "An account with this email already exists. You can find it under Accounts page."
                        case .Mavinote(.Message("expired_code")):
                            error = "5 minutes waiting is timed out. Please try again."
                        case .Mavinote(.Message("invalid_code")):
                            error = "You have entered invalid code. Please check the verification code."
                        default: appState.handleError(e)
                        }
                    }

                    inProgress = false
                }
            }) {
                Text("Verify Code")
                    .frame(maxWidth: .infinity)
                    .foregroundColor(.white)
            }
            .frame(maxWidth: .infinity)
            .padding(12)
            .background(inProgress ? .gray : .blue)
            .disabled(inProgress)
            .cornerRadius(8)
            .padding(.all, 12)
        }
        .navigationTitle("Verify Account")
        .alert(item: $error) { error in
            Alert(
                title: Text(""),
                message: Text(error),
                dismissButton: .default(Text("Ok"))
            )
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
        }
    }
}

struct ShowPublicKey_Preview: PreviewProvider {
    static var previews: some View {
        NavigationView {
            ShowPublicKeyView(email: "email@email.com", token: "TOKEN")
        }
    }
}

struct SendVerificationCode_Preview: PreviewProvider {
    static var previews: some View {
        NavigationView {
            SendVerificationCodeView()
        }
    }
}

struct AccountAddVerifyCode_Preview: PreviewProvider {
    static var previews: some View {
        NavigationView {
            VerifyCodeView(email: "email@email.com")
        }
    }
}
