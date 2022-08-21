import SwiftUI

struct FolderCreateView : View {
    var onClose: () -> ()

    @EnvironmentObject var appState: AppState

    @State var tasks: [Task<(), Never>] = []
    @State var accounts: [Account] = []
    @State var inProgress = false
    @State var error: String?


    var body: some View {
        _FolderCreateView(
            onCreate: { name in
                if (inProgress) {
                    return
                }

                if name.isEmpty {
                    error = "Please specify a folder name"
                    return
                }

                error = nil
                inProgress = true

                Task {
                    do {
                        try await NoteViewModel().createFolder(name)
                        onClose()
                    } catch {
                        print("failed to create folder", error)
                    }

                    inProgress = false
                }
            },
            accounts: $accounts,
            inProgress: $inProgress,
            error: $error
        )
        .navigationTitle("Create Folder")
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
        .onDisappear {
            tasks.forEach { task in task.cancel() }
        }
    }
}

private struct _FolderCreateView: View {
    let onCreate: (_ name: String) -> ()

    @Binding var accounts: [Account]
    @Binding var inProgress: Bool
    @Binding var error: String?

    @State var name = ""
    @State var accountId: Int32?

    var body: some View {
        VStack(alignment: .leading) {
            Text("Folder Name")
                .font(.title2)
                .padding(.bottom, 8)
                .padding(.top, 32)

            TextField("Name", text: $name)
                .textContentType(.name)
                .padding(10)
                .accentColor(.red)
                .background(Color(red: 242 / 255, green: 242 / 255, blue: 242 / 255))
                .cornerRadius(5)
                .padding(.bottom, 12)

            if accounts.count > 1 {
                 Text("Account this folder be created in")
                    .font(.title3)
                    .padding(.top, 32)

                Picker(selection: $accountId, label: Text("Account")) {
                    ForEach(accounts, id: \.self.id) { account in
                        Text(account.name)
                            .tag(account.id)
                    }
                }.pickerStyle(.menu)
            }

            Spacer()

            if let error = error {
                Text(error)
                    .foregroundColor(.red)
            }

            Button(action: {
                onCreate(name)
            }) {
                Text("Create Folder")
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
                onCreate: { name in },
                accounts: .constant(accounts),
                inProgress: .constant(false),
                error: .constant("Please specify a name")
            )
                .navigationTitle("Create Folder")
        }
    }
}
