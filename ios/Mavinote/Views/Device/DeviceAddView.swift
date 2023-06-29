import SwiftUI

struct DeviceAddView: View {
    enum ValidationErrors {
        case InvalidPublicKey
    }

    let accountId: Int32

    @EnvironmentObject var appState: AppState

    @State var publicKey: String = ""
    @State var validationErrors = Set<ValidationErrors>()
    @State var error: String?
    @State var inProgress = false

    var body: some View {
        VStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 32.0) {
                    Text("Cryptographic keys, called Public Key, are used to identify devices.")

                    Text("In order to add a new device into this account, you first need to choose Add an Existing Account in Add Account page on new device.")

                    Text("Then you need to type the Public Key of the new device below and tap Add Device.")

                    VStack(alignment: .leading) {
                        Text("Device Public Key")
                            .font(.callout)

                        TextField("Public Key", text: $publicKey)
                            .textInputAutocapitalization(.never)
                            .padding(12)
                            .background(InputBackground)
                            .cornerRadius(8)

                        if validationErrors.contains(.InvalidPublicKey) {
                            Text("Please specify a valid Public Key")
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

                if publicKey.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
                    validationErrors.insert(.InvalidPublicKey)
                }

                if !validationErrors.isEmpty {
                    return
                }

                inProgress = true

                Task {
                    switch await AccountViewModel.addDevice(accountId, publicKey) {
                    case .success(_):
                        appState.emit(.ShowMessage("Device is added successfully"))
                        appState.navigate(route: .Accounts)
                    case .failure(let e):
                        switch e {
                        case .Mavinote(.Message("item_not_found")):
                            error = "Public Key is not found"
                        case .Mavinote(.Message("device_already_exists")):
                            error = "Device with this public key is already added"
                        case .Mavinote(.Message("expired_pubkey")):
                            error = "5 minutes waiting is timed out. Please try the steps on new device again."
                        default: e.handle(appState)
                        }
                    }

                    inProgress = false
                }
            }) {
                Text("Add Device")
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
        .navigationTitle("Add Device")
        .alert(item: $error) { error in
            Alert(
                title: Text(""),
                message: Text(error),
                dismissButton: .default(Text("Ok"))
            )
        }
    }
}

struct DeviceAdd_Preview: PreviewProvider {
    static var previews: some View {
        NavigationView {
            DeviceAddView(accountId: 1)
        }
    }
}
