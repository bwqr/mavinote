import SwiftUI

struct FoldersView: View {
    @State var folderCreateActive = false
    @State var folders: [Folder] = []
    
    var body: some View {
        
        List(folders) { folder in
            NavigationLink(destination: {
                NotesView(folder.id)
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

        .onAppear {
            Task {
                do {
                    folders = try await NoteViewModel().folders()
                } catch {
                    print("failed to fetch folders", error)
                }
            }
        }
    }
}
