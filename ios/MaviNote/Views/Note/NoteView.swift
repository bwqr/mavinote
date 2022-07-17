import SwiftUI

struct NoteView : View {
    let folderId: Int32
    var noteId: Int32?
    @State var task: Task<(), Never>?
    @State var text = ""
    @State var updateNote: Bool = false
    @EnvironmentObject var appState: AppState

    var body: some View {
        VStack {
            Text("Note")
            TextField("Your note goes here", text: $text)
        }.onAppear {
            guard let noteId = noteId else {
                return
            }

            task = Task {
                do {
                    if let note = try await NoteViewModel().note(noteId) {
                        updateNote = true
                        text = note.text
                    }
                } catch {
                    print("failed to fetch note", error)
                }
            }
        }.onDisappear {
            task?.cancel()

            Task {
                do {
                    if let noteId = noteId {
                        if updateNote {
                            try await NoteViewModel().updateNote(noteId, text)
                        }
                    } else {
                        let noteId = try await NoteViewModel().createNote(folderId)
                        try await NoteViewModel().updateNote(noteId, text)
                    }
                } catch {
                    print("failed to update or create note", error)
                }
            }
        }
    }
}
