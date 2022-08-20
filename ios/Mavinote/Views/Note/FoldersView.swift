import SwiftUI

struct FoldersView: View {
    @State var task: Task<(), Never>?
    @State var folders: [Folder] = []
    @State var showFolderCreate = false

    @EnvironmentObject var appState: AppState

    var body: some View {
        NavigationView {
            List(folders) { folder in
                NavigationLink(destination: {
                    NotesView(folderId: folder.id)
                        .navigationTitle(folder.name)
                }) {
                    Text(folder.name)
                        .padding()
                }
            }
            .toolbar {
                NavigationLink(destination: FolderCreateView(onClose: { showFolderCreate = false }), isActive: $showFolderCreate) {
                    Text("Add Folder")
                }
            }
            .navigationBarTitle("Folders")
            .onAppear {
                task = Task {
                    let stream = NoteViewModel().folders()

                    for await result in stream {
                        switch result {
                        case .success(let f): folders = f
                        case .failure(_): appState.navigate(Screen.Login)
                        }
                    }
                }
            }
            .onDisappear {
                if let task = task {
                    task.cancel()
                }
            }
        }
    }
}
