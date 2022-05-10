import SwiftUI

struct NoteView : View {
    let folderId: Int32
    var noteId: Int32?
    @State var text = ""
    
    var body: some View {
        VStack {
            Text("Note")
            TextEditor(text: $text)
        }.onAppear {
            guard let noteId = noteId else {
                return
            }

            Task {
                do {
                    if let note = try await NoteViewModel().note(noteId) {
                        text = note.text
                    }
                } catch {
                    print("failed to fetch note", error)
                }
            }
        }.onDisappear {
            Task {
                do {
                    var noteId = noteId

                    if noteId == nil {
                        noteId = try await NoteViewModel().createNote(folderId)
                    }

                    if let noteId = noteId {
                        try await NoteViewModel().updateNote(noteId, text)
                    }
                } catch {
                    print("failed to update or create note", error)
                }
            }
        }
    }
}
