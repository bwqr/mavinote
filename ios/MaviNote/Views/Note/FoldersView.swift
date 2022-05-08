import SwiftUI

struct FoldersView: View {
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
            Task {
                do {
                    folders = try await NoteViewModel().folders()
                } catch {
                    appState.navigate(Screen.Login)
                    print("failed to fetch folders", error)
                }
            }
        }
    }
}
