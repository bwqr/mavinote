import SwiftUI
import AsyncAlgorithms

private struct AccountWithFolders: Identifiable {
    let account: Account
    let folders: [Folder]

    var id: Int32 { get { account.id } }
}

struct FoldersView: View {
    @State var tasks: [Task<(), Never>] = []
    @State private var accounts: [AccountWithFolders] = []

    @EnvironmentObject var appState: AppState

    var body: some View {
        _FoldersView(accounts: $accounts)
            .onAppear {
                tasks.append(Task {
                    for await result in combineLatest(AccountViewModel.accounts(), NoteViewModel.folders()) {
                        switch result {
                        case (.success(let a), .success(let f)):
                            accounts = a.map { account in
                                AccountWithFolders(
                                    account: account,
                                    folders: f.filter{ folder in folder.accountId == account.id }
                                )
                            }
                        case (.failure(let e), _): e.handle(appState)
                        case (_, .failure(let e)): e.handle(appState)
                        }
                    }
                })
            }
            .onDisappear {
                tasks.forEach { $0.cancel() }
            }
    }
}

private struct _FoldersView : View {
    @Binding var accounts: [AccountWithFolders]

    var body: some View {
        NavigationView {
            VStack {
                List(accounts) { accountWithFolder in
                    Section(header: HStack {
                        Text(accountWithFolder.account.name)
                        Spacer()
                        Text("\(accountWithFolder.folders.count)")
                            .font(.footnote)
                    }) {
                        ForEach(accountWithFolder.folders) { folder in
                            NavigationLink(destination: NotesView(folderName: folder.name, folderId: folder.id)) {
                                Text(folder.name)
                            }
                        }
                    }
                    .padding(12)
                }

                HStack {
                    Spacer()
                    NavigationLink(
                        destination: FolderCreateView()
                    ) {
                        Image(systemName: "folder.badge.plus")
                            .padding(EdgeInsets(top: 2, leading: 12, bottom: 12, trailing: 24))
                            .foregroundColor(.blue)
                    }
                }
            }
            .navigationTitle("Folders")
            .toolbar {
                NavigationLink(destination: AccountsView()) {
                    Text("Accounts")
                }
            }
        }
    }
}

struct FoldersView_Preview: PreviewProvider {
    static  var previews: some View {
        let accounts = [
            AccountWithFolders(
                account: Account(id: 1, name: "Local", kind: .Local),
                folders: [
                    Folder(id: 1, accountId: 1, remoteId: nil, name: "Favorites", state: .Clean),
                    Folder(id: 2, accountId: 1, remoteId: nil, name: "Todos", state: .Clean),
                    Folder(id: 4, accountId: 1, remoteId: nil, name: "Projects", state: .Clean),
                    Folder(id: 5, accountId: 1, remoteId: nil, name: "Kernel", state: .Clean),
               ]
            ),
            AccountWithFolders(
                account: Account(id: 2, name: "Mavinote", kind: .Mavinote),
                folders: [
                    Folder(id: 3, accountId: 2, remoteId: nil, name: "Race Cars", state: .Clean),
               ]
            )
        ]

        _FoldersView(accounts: .constant(accounts))
    }
}
