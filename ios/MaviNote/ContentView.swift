import SwiftUI

struct ContentView: View {
    @State var folders: [Folder] = []
    
    var body: some View {
        List(folders) { folder in
             Text(folder.name)
                .padding()
        }
        .onAppear {
            Task {
                do {
                    folders = try await NoteViewModel().folders()
                } catch {
                    print("failed to fetch folders", error)
                }
            }
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
