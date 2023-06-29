import SwiftUI

struct AccountCloseView: View {
    let accountId: Int32

    @EnvironmentObject var appState: AppState
    @State var mavinote: Mavinote?

    var body: some View {
        ZStack {
            if let mavinote = mavinote {
                SendCodeView(accountId: accountId, email: mavinote.email)
            }
        }
        .navigationTitle("Close Account")
        .onAppear {
            Task {
                switch await AccountViewModel.mavinoteAccount(accountId) {
                case .success(let m): mavinote = m
                case .failure(let e): appState.handleError(e)
                }
            }
        }
    }
}

struct SendCodeView: View {
    let accountId: Int32
    let email: String
    @EnvironmentObject var appState: AppState
    @State var showVerifyCode = false
    @State var inProgress = false

    var body: some View {
        VStack(alignment: .leading) {
            NavigationLink(isActive: $showVerifyCode, destination: { VerifyCodeView(accountId: accountId, email: email) }) {
                EmptyView()
            }
            VStack(alignment: .leading, spacing: 32.0) {
                Text("Are sure about closing account?")

                Text("In order to close the account, we will send a verification code to \(email) email address.")
            }
            .padding(.all, 12)

            Spacer()

            Button(action: {
                if inProgress {
                    return
                }

                inProgress = true

                Task {
                    switch await AccountViewModel.sendAccountCloseCode(accountId) {
                    case .success(_):
                        showVerifyCode = true
                    case .failure(let e):
                        appState.handleError(e)
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
        .navigationTitle("Close Account")
    }
}


private struct VerifyCodeView: View {
    enum ValidationErrors {
        case InvalidCode
    }

    let accountId: Int32
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

                    Text("Please enter verification code to close your account.")

                    VStack(alignment: .leading) {
                        Text("Code")
                            .font(.callout)

                        TextField("Code", text: $code)
                            .textInputAutocapitalization(.never)
                            .textContentType(.oneTimeCode)
                            .keyboardType(.numberPad)
                            .padding(12)
                            .background(InputBackground)
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
                    switch await AccountViewModel.closeAccount(accountId, code) {
                    case .success(_):
                        appState.emit(.ShowMessage("Account is closed"))
                        appState.navigate(route: .Accounts)
                    case .failure(let e):
                        switch e {
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
                Text("Close Account")
                    .frame(maxWidth: .infinity)
                    .foregroundColor(.white)
            }
            .frame(maxWidth: .infinity)
            .padding(12)
            .background(inProgress ? .gray : .red)
            .disabled(inProgress)
            .cornerRadius(8)
            .padding(.all, 12)
        }
        .navigationTitle("Verify Code")
        .alert(item: $error) { error in
            Alert(
                title: Text(""),
                message: Text(error),
                dismissButton: .default(Text("Ok"))
            )
        }
    }
}

struct SendCode_Preview: PreviewProvider {
    static var previews: some View {
        NavigationView {
            SendCodeView(accountId: 1, email: "email@email.com")
        }
    }
}

struct AccountCloseVerifyCode_Preview: PreviewProvider {
    static var previews: some View {
        NavigationView {
            VerifyCodeView(accountId: 1, email: "email@email.com")
        }
    }
}
