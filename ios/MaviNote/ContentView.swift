import SwiftUI

struct ContentView: View {
    var body: some View {
        NavigationView {
            FoldersView()
                .navigationTitle("Folders")
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
