import SwiftUI
import AsyncAlgorithms

struct AccountWithFolders {
    let account: Account
    let folders: [Folder]
}

struct FoldersView: View {
    @State var tasks: [Task<(), Never>] = []
    @State var accounts: [AccountWithFolders] = []

    @EnvironmentObject var appState: AppState

    var body: some View {
        _FoldersView(accounts: $accounts)
            .onAppear {
                tasks.append(Task {
                    for await result in combineLatest(NoteViewModel().accounts(), NoteViewModel().folders()) {
                        switch result {
                        case (.success(let a), .success(let f)):
                            accounts = a.map { account in
                                AccountWithFolders(account: account, folders: f.filter{ folder in folder.accountId == account.id })
                            }
                        default: appState.navigate(Screen.Login)
                        }
                    }
                })
            }
            .onDisappear {
                tasks.forEach { task in task.cancel() }
            }
    }
}

struct _FoldersView : View {
    @State var showFolderCreate = false

    @Binding var accounts: [AccountWithFolders]

    var body: some View {
        NavigationView {
        }
    }
}

struct FoldersView_Preview: PreviewProvider {
    static  var previews: some View {
        _FoldersView(accounts: .constant([]))
    }
}
