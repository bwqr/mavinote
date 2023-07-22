import SwiftUI

struct HelpView: View {
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 32.0) {
                Text(
                    "Mavinote is in beta stage, meaning, it is not fully stable yet and subject to frequent changes." +
                    " Right now, there is not much knowledge written in somewhere except the repository and the application themselves."
                )

                Text(
                    "For any kind of help you need, please take a look into the repository issues or discussions." +
                    " If you are not able to find a similar topic to yours, please create a new one." +
                    " You can find the repository in the link below."
                )

                Text("https://github.com/bwqr/mavinote")
            }
            .padding(.all, 12)
        }
        .navigationTitle("Help")
    }
}
