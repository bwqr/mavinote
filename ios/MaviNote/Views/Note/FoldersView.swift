import SwiftUI

struct FoldersView: View {
    @State var task: Task<(), Never>?
    @State var folderCreateActive = false
    @State var folders: [Folder] = []
    @EnvironmentObject var appState: AppState
    
    var body: some View {
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
            NavigationLink(destination: FolderCreateView(viewActive: $folderCreateActive), isActive: $folderCreateActive) {
                Text("Add Folder")
            }
        }
        .navigationBarBackButtonHidden(true)
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
