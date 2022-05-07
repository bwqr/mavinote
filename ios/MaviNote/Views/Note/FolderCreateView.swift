import SwiftUI

struct FolderCreateView : View {
    @Binding var viewActive: Bool
    @State var inProgress = false
    @State var name = ""
    
    var body: some View {
        if (inProgress) {
            Text("In progress")
        }
        
        TextField("Name", text: $name)
        
        Button("Add") {
            if (inProgress) {
                return
            }
            
            inProgress = true
            
            Task {
                do {
                    try await NoteViewModel().createFolder(name)
                    viewActive = false
                } catch {
                    print("failed to create folder", error)
                }
                
                inProgress = false
            }
        }
    }
}
