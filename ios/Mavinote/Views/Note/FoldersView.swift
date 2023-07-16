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
        _FoldersView(
            accounts: $accounts,
            onRefresh: {
                let _ = await NoteViewModel.sync()
            }
        )
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
                        case (.failure(let e), _): appState.handleError(e)
                        case (_, .failure(let e)): appState.handleError(e)
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
    @EnvironmentObject var appState: AppState
    @Binding var accounts: [AccountWithFolders]

    let onRefresh: () async -> ()

    var body: some View {
        VStack {
            List(accounts) { accountWithFolder in
                Section(
                    content: {
                        ForEach(accountWithFolder.folders) { folder in
                            NavigationLink(destination: NotesView(folderName: folder.name, folderId: folder.id)) {
                                Text(folder.name)
                            }
                        }
                    },
                    header: {
                        HStack {
                            Text(accountWithFolder.account.name)
                            Spacer()
                            Text("\(accountWithFolder.folders.count)")
                                .font(.footnote)
                        }
                    },
                    footer: {
                        if accountWithFolder.folders.count == 0 {
                            Text("There is no folder in this account")
                        }
                    }
                )
                .padding(12)
            }
            .refreshable {
                await onRefresh()
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
            NavigationLink(
                destination: AccountsView(),
                tag: Route.Accounts,
                selection: $appState.activeRoute
            ) {
                Text("Accounts")
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
            AccountWithFolders(account: Account(id: 3, name: "Remote", kind: .Mavinote), folders: []),
            AccountWithFolders(
                account: Account(id: 2, name: "Mavinote", kind: .Mavinote),
                folders: [
                    Folder(id: 3, accountId: 2, remoteId: nil, name: "Race Cars", state: .Clean),
               ]
            ),
        ]

        _FoldersView(accounts: .constant(accounts), onRefresh: { })
            .environmentObject(AppState())
    }
}
