import SwiftUI

struct NotesView: View {
    let folderId: Int32
    @State var notes: [Note] = []

    var body: some View {
        List(notes) { note in
            NavigationLink(destination: {
                NoteView(folderId: folderId, noteId: note.id)
            }) {
                Text(note.title)
            }
        }.onAppear {
            Task {
                do {
                    notes = try await NoteViewModel().noteSummaries(self.folderId)
                } catch {
                    print("failed to fetch noteSummaries", error)
                }
            }
        }.toolbar {
            NavigationLink(destination: NoteView(folderId: folderId, noteId: nil)) {
                Text("Add Note")
            }
        }
    }
}
