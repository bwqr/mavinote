import SwiftUI

struct NoteView : View {
    let folderId: Int32
    var noteId: Int32?
    @State var task: Task<(), Never>?
    @State var text = ""
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
                let stream = NoteViewModel().note(noteId)

                for await result in stream {
                    switch result {
                    case .success(let n): if let n = n {
                        text = n.text
                    }
                    case .failure(_): appState.navigate(Screen.Login)
                    }
                }
            }
        }.onDisappear {
            if let task = task {
                task.cancel()

                Task {
                    do {
                        var noteId = noteId

                        if noteId == nil {
                            noteId = try await NoteViewModel().createNote(folderId)
                        }

                        if let noteId = noteId {
                            try await NoteViewModel().updateNote(noteId, folderId, text)
                        }
                    } catch {
                        print("failed to update or create note", error)
                    }
                }
            }
        }
    }
}
