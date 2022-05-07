import SwiftUI

struct NotesView: View {
    private let folderId: Int32
    @State var notes: [Note] = []

    init(_ folderId: Int32) {
        self.folderId = folderId
    }

    var body: some View {
        List(notes) { note in
            NavigationLink(destination: {
                NoteView(noteId: note.id)
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
            Button("Somehow add new note") { }
        }
    }
}
