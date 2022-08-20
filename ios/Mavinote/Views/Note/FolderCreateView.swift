import SwiftUI

struct FolderCreateView : View {
    var onClose: () -> ()

    @EnvironmentObject var appState: AppState

    @State var tasks: [Task<(), Never>] = []
    @State var accounts: [Account] = []
    @State var inProgress = false


    var body: some View {
        _FolderCreateView(
            onCreate: { name in
                if (inProgress) {
                    return
                }

                inProgress = true

                Task {
                    do {
                        try await NoteViewModel().createFolder(name)
                        onClose()
                    } catch {
                        print("failed to create folder", error)
                    }

                    inProgress = false
                }
            },
            accounts: $accounts,
            inProgress: $inProgress
        )
        .navigationTitle("Create Folder")
        .onAppear {
            tasks.append(Task {
                let stream = NoteViewModel().accounts()

                for await result in stream {
                    switch result {
                    case .success(let a): accounts = a
                    case .failure(_): appState.navigate(Screen.Login)
                    }
                }
            })
        }
    }
}

private struct _FolderCreateView: View {
    let onCreate: (_ name: String) -> ()

    @Binding var accounts: [Account]
    @Binding var inProgress: Bool

    @State var name = ""

    var body: some View {
        VStack(alignment: .leading) {
            Text("Folder Name")
                .font(.title2)
                .padding(.bottom, 8)

            TextField("Name", text: $name)
                .textContentType(.name)
                .padding(10)
                .accentColor(.red)
                .background(Color(red: 242 / 255, green: 242 / 255, blue: 242 / 255))
                .cornerRadius(5)
                .padding(.bottom, 12)

            Spacer()

            Button("Add") {
                onCreate(name)
            }
            .padding(12)
        }
        .padding([.leading, .trailing], 12)
    }
}

struct FolderCreateView_Previews: PreviewProvider {
    static var previews: some View {
        let accounts = [
            Account(id: 1, name: "Local", kind: .Local)
        ]

        NavigationView {
            _FolderCreateView(onCreate: { name in }, accounts: .constant(accounts), inProgress: .constant(false))
                .navigationTitle("Create Folder")
        }
    }
}
