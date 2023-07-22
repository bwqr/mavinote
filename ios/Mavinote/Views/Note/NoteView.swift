import SwiftUI

struct NoteView : View {
    let folderId: Int32
    let noteName: String
    let noteId: Int32?

    @State var tasks: [Task<(), Never>] = []
    @State var text: String = ""
    @State var modified = false
    @State var deleting = false

    @EnvironmentObject var appState: AppState
    @Environment(\.dismiss) var dismiss: DismissAction

    var body: some View {
        _NoteView(
            text: $text,
            textViewDelegate: TextViewDelegate(onTextChange: { text in
                self.text = text
                self.modified = true
            }),
            onDelete: {
                if deleting {
                    return
                }

                deleting = true

                tasks.append(Task {
                    if let noteId = noteId {
                        switch await NoteViewModel.deleteNote(noteId) {
                        case .success(_): dismiss()
                        case .failure(let e):
                            appState.handleError(e)
                        }
                    } else {
                        dismiss()
                    }

                    deleting = false
                })
            }
        )
        .navigationTitle(noteName)
        .onAppear {
            guard let noteId = noteId else {
                return
            }

            tasks.append(Task {
                switch await NoteViewModel.note(noteId) {
                case .success(let note):
                    if let note = note {
                        text = note.text
                        modified = false
                    } else {
                        print("WARNING: Note is not found")
                    }
                case .failure(let e): appState.handleError(e)
                }
            })
        }.onDisappear {
            tasks.forEach { $0.cancel() }

            if deleting {
                return
            }

            Task {
                if let noteId = noteId, modified {
                    if case .failure(let e) = await NoteViewModel.updateNote(noteId, text) {
                        appState.handleError(e)
                    }
                } else if noteId == nil && !text.isEmpty {
                    if case .failure(let e) = await NoteViewModel.createNote(folderId, text) {
                        appState.handleError(e)
                    }
                }
            }
        }
    }
}

private struct _NoteView : View {
    @Binding var text: String

    let textViewDelegate: TextViewDelegate
    let onDelete: () -> ()

    @State var showEdit = false

    var body: some View {
        TextView() {
            $0.text = text
            $0.font = UIFont.systemFont(ofSize: 18)
            $0.layer.borderColor = CGColor(gray: 0.5, alpha: 1.0)
            $0.layer.borderWidth = 0.5
            $0.layer.cornerRadius = 8.0
            $0.textContainerInset = UIEdgeInsets(top: 16, left: 8, bottom: 8, right: 8)
            $0.delegate = textViewDelegate
        }
        .cornerRadius(8)
        .padding(8)
        .toolbar {
            Button("Edit") {
                showEdit = true
            }
            .confirmationDialog("Edit Account", isPresented: $showEdit)  {
                Button("Delete", role: .destructive) {
                    onDelete()
                }
            }
        }

    }
}

// https://stackoverflow.com/a/58639072
private struct TextView: UIViewRepresentable {
    typealias UIViewType = UITextView
    var configuration = { (view: UIViewType) in }

    func makeUIView(context: Context) -> UITextView {
        UIViewType()
    }

    func updateUIView(_ uiView: UITextView, context: Context) {
        configuration(uiView)
    }
}

private class TextViewDelegate : NSObject, UITextViewDelegate {
    let onTextChange: (_ text: String) -> ()

    init(onTextChange: @escaping (_ text: String) -> ()) {
        self.onTextChange = onTextChange
    }

    func textViewDidChange(_ textView: UITextView) {
        onTextChange(textView.text)
    }
}

struct NoteView_Preview : PreviewProvider {
    static var previews: some View {
        let note = Note(id: 1, folderId: 1, remoteId: 1, commit: 1, name: "My Note", text: "Little note in code", state: .Clean)

        NavigationView {
            _NoteView(
                text: .constant(note.text),
                textViewDelegate: TextViewDelegate(onTextChange: { _ in }),
                onDelete: { }
            )
            .navigationTitle(note.name)
        }
    }
}
