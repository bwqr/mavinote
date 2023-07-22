import SwiftUI

struct FolderCreateView : View {
    @EnvironmentObject var appState: AppState

    @Environment(\.dismiss) var dismiss: DismissAction

    @State var tasks: [Task<(), Never>] = []
    @State var accounts: [Account]?
    @State var inProgress = false
    @State var error: String?

    var body: some View {
        SafeContainer(value: $accounts) { accounts in
            _FolderCreateView(
                onCreate: { accountId, name in
                    if (inProgress) {
                        return
                    }

                    if name.isEmpty {
                        error = "Please specify a folder name"
                        return
                    }

                    guard let accountId = accountId else {
                        error = "Please select an account"
                        return
                    }

                    error = nil
                    inProgress = true

                    tasks.append(Task {
                        switch await NoteViewModel.createFolder(accountId, name) {
                        case .success(_): dismiss()
                        case .failure(let e): appState.handleError(e)
                        }

                        inProgress = false
                    })
                },
                accounts: accounts,
                inProgress: $inProgress,
                error: $error
            )
        }
        .navigationTitle("Create Folder")
        .onAppear {
            tasks.append(Task {
                let stream = AccountViewModel.accounts()

                for await result in stream {
                    switch result {
                    case .success(let a): accounts = a
                    case .failure(let e): appState.handleError(e)
                    }
                }
            })
        }
        .onDisappear {
            tasks.forEach { task in task.cancel() }
        }
    }
}

private struct _FolderCreateView: View {
    typealias OnCreate = (_ accountId: Int32?, _ name: String) -> ()

    let onCreate: OnCreate

    @Binding var accounts: [Account]
    @Binding var inProgress: Bool
    @Binding var error: String?

    @State var name = ""
    @State var accountId: Int32?

    var body: some View {
        VStack {
            ScrollView {
                VStack(alignment: .leading) {
                    Text("Folder Name")
                        .font(.callout)
                        .padding(.bottom, 8)
                        .padding(.top, 16)

                    TextField("Name", text: $name)
                        .textFieldStyle(.roundedBorder)
                        .cornerRadius(5)
                        .padding(.bottom, 16)

                    if accounts.count > 1 {
                        Text("Account this folder will be created in")
                            .font(.callout)
                            .padding(.bottom, 8)

                        ForEach(accounts, id: \.self.id) { account in
                            HStack {
                                Image(systemName: accountId == account.id ? "circle.inset.filled" : "circle")
                                Text(account.name).tag(account.id as Int32?)
                            }
                            .onTapGesture {
                                accountId = account.id
                            }
                        }
                        .padding(.bottom, 12)
                    }

                    if let error = error {
                        Text(error)
                            .foregroundColor(.red)
                    }
                }
                .padding([.leading, .trailing], 12)
            }

            Button(action: {
                onCreate(accountId, name)
            }) {
                Text("Create Folder")
                    .frame(maxWidth: .infinity)
                    .foregroundColor(.white)
            }
            .frame(maxWidth: .infinity)
            .padding(12)
            .background(.blue)
            .cornerRadius(8)
            .padding(12)
        }
        .onAppear {
            accountId = accounts.first?.id
        }
    }
}

struct FolderCreateView_Previews: PreviewProvider {
    static var previews: some View {
        let accounts = [
            Account(id: 1, name: "Local", kind: .Local),
            Account(id: 2, name: "First Mavinote", kind: .Mavinote),
            Account(id: 3, name: "Second Mavinote", kind: .Mavinote),
        ]

        NavigationView {
            _FolderCreateView(
                onCreate: { accountId, name in },
                accounts: .constant(accounts),
                inProgress: .constant(false),
                error: .constant("Please specify a name")
            )
            .navigationTitle("Create Folder")
        }
    }
}
