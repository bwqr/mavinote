import SwiftUI

struct NotesView: View {
    let folderName: String
    let folderId: Int32
    @State var tasks: [Task<(), Never>] = []
    @State var folder: Folder?
    @State var notes: [Note] = []
    @State var inProgress = false

    @EnvironmentObject var appState: AppState

    @Environment(\.dismiss) var dismiss: DismissAction

    var body: some View {
        ZStack {
            if let folder = folder {
                _NotesView(
                    folder: folder,
                    notes: notes,
                    onDelete: {
                        if inProgress {
                            return
                        }

                        inProgress = true

                        tasks.append(Task {
                            switch await NoteViewModel.deleteFolder(folderId) {
                            case .success(_):
                                appState.emit(.ShowMessage("Folder is deleted"))
                                dismiss()
                            case .failure(let e): appState.handleError(e)
                            }

                            inProgress = false
                        })
                    }
                )

            }
        }
        .navigationTitle(folderName)
        .onAppear {
            tasks.append(Task {
                let stream = NoteViewModel.notes(self.folderId)

                for await result in stream {
                    switch result {
                    case .success(let n): notes = n
                    case .failure(let e): appState.handleError(e)
                    }
                }
            })

            tasks.append(Task {
                switch await NoteViewModel.folder(folderId) {
                case .success(let f): folder = f
                case .failure(let e): appState.handleError(e)
                }
            })
        }
        .onDisappear {
            tasks.forEach { $0.cancel() }
        }
    }
}

private struct _NotesView : View {
    let folder: Folder
    let notes: [Note]

    let onDelete: () -> ()

    @State var showEdit = false

    var body: some View {
        VStack {
            if notes.count == 0 {
                Text("There is no note in this folder")
                Spacer()
            } else {
                List(notes, id: \.self.id) { note in
                    NavigationLink(destination: NoteView(folderId: folder.id, noteName: note.name, noteId: note.id)) {
                        Text(note.name)
                    }
                    .padding(12)
                }
            }

            HStack {
                Spacer()
                NavigationLink(
                    destination: NoteView(folderId: folder.id, noteName: "New Note", noteId: nil)
                ) {
                    Image(systemName: "square.and.pencil")
                        .padding(EdgeInsets(top: 2, leading: 12, bottom: 12, trailing: 24))
                        .foregroundColor(.blue)
                }
            }

        }
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

struct NotesView_Preview : PreviewProvider {
    static var previews : some View {
        let folder = Folder(id: 1, accountId: 1, remoteId: nil, name: "My Folder", state: .Clean)
        let notes = [
            Note(id: 1, folderId: 1, remoteId: nil, commit: 1, name: "Little Note", text: "Empty text", state: .Clean),
            Note(id: 2, folderId: 1, remoteId: nil, commit: 1, name: "Hacky Solutions", text: "Empty text", state: .Clean),
            Note(id: 3, folderId: 1, remoteId: nil, commit: 1, name: "No Surprise", text: "Empty text", state: .Clean)
        ]

        NavigationView {
            _NotesView(
                folder: folder,
                notes: notes,
                onDelete: { }
            )
            .navigationTitle(folder.name)
        }
    }
}
