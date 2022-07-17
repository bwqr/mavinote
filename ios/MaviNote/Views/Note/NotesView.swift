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
                if let title = note.title {
                    Text(title)
                } else {
                    Text("New title")
                }
            }
        }.onAppear {
            task = Task {
                let stream = NoteViewModel().noteSummaries(self.folderId)

                for await result in stream {
                    switch result {
                    case .success(let n): notes = n
                    case .failure(let error): debugPrint("failed to fetch note summaries", error)
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
