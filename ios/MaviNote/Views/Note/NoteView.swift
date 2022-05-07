import SwiftUI

struct NoteView : View {
    let noteId: Int32
    @State var text = ""
    
    var body: some View {
        VStack {
            TextEditor(text: $text)
        }.onAppear {
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
                    try await NoteViewModel().updateNote(noteId, text)
                } catch {
                    print("failed to update note", error)
                }
            }
        }
    }
}
