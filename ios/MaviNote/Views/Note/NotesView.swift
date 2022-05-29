import SwiftUI

struct NotesView: View {
    let folderId: Int32
    @State var task: Task<(), Never>?
    @State var notes: [Note] = []
    @EnvironmentObject var appState: AppState

    var body: some View {
        List(notes) { note in
            NavigationLink(destination: {
                NoteView(folderId: folderId, noteId: note.id)
            }) {
                Text(note.title)
            }
        }.onAppear {
            task = Task {
                let stream = NoteViewModel().noteSummaries(self.folderId)

                for await result in stream {
                    switch result {
                    case .success(let n): notes = n
                    case .failure(_): appState.navigate(Screen.Login)
                    }
                }
            }
        }.onDisappear {
            task?.cancel()
        }.toolbar {
            NavigationLink(destination: NoteView(folderId: folderId, noteId: nil)) {
                Text("Add Note")
            }
        }
    }
}
