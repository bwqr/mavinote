import SwiftUI

struct WelcomeView: View {
    var body: some View {
        WelcomeMavinote()
    }
}

private struct WelcomeMavinote: View {
    var body: some View {
        VStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 32.0) {
                    Text("Mavinote is a simple, open-source, secure, and multi device note taking application.")

                    Text("You can take notes that reside only on your device, or create a Mavinote account to store them in the cloud.")

                    Text("You can access your notes from other devices as well by adding your existing account into them.")

                    Text("All the notes stored in the cloud are encrypted and only readable by your devices.")

                    Text("Did we say that Mavinote is open-source. You can reach out the source code under the repository written below.")

                    Text("https://github.com/bwqr/mavinote")
                }
                .padding(.all, 12)
            }
            NavigationLink(destination: AccountAndDevice()) {
                Text("Learn About Accounts and Devices")
                    .frame(maxWidth: .infinity)
                    .foregroundColor(.white)
            }
                .frame(maxWidth: .infinity)
                .padding(12)
                .background(.blue)
                .cornerRadius(8)
                .padding(.all, 12)
        }
        .navigationTitle("Welcome to Mavinote")
        .navigationBarBackButtonHidden()
    }
}

private struct AccountAndDevice: View {
    var body: some View {
        VStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 32.0) {
                    Text(
                        "Mavinote has an hierarchy defined by accounts, devices, folders and notes." +
                                " An account is an entity that stores the folders and notes under itself." +
                                " Mavinote creates an account by default, called Local, when you launch it for the first time." +
                                " This account enables you to take notes that will reside only on your device."
                    )

                    Text(
                        "A device is responsible for managing the accounts." +
                                " In addition to Local account, you can add multiple accounts, called Mavinote, to your devices." +
                                " These Mavinote accounts enable you to take notes that will be synchronized between your other devices." +
                                " In order to synchronize the notes, you need to add same Mavinote account to your devices."
                    )
                }
                .padding(.all, 12)
            }

            NavigationLink(destination: FolderAndNote()) {
                Text("Learn About Folders and Notes")
                    .frame(maxWidth: .infinity)
                    .foregroundColor(.white)
            }
                .frame(maxWidth: .infinity)
                .padding(12)
                .background(.blue)
                .cornerRadius(8)
                .padding(.all, 12)
        }
        .navigationTitle("Manage Accounts and Devices")
    }
}

private struct FolderAndNote: View {
    @EnvironmentObject var appState: AppState

    var body: some View {
        VStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 32.0) {
                    Text(
                        "In Mavinote application, you can create folders and put notes into them." +
                                " In order to create a note, you first need to create a folder." +
                                " Folders can be created by specifying a name."
                    )

                    Text(
                        "If you have more than one account, you also need to choose an account while creating a folder." +
                                " You can create notes after navigating to any folder." +
                                " Right now, Mavinote application only supports taking plain text notes." +
                                " Note editing will be improved with upcoming releases."
                    )

                    Text("Now you are ready to dive into Mavinote application. Go ahead and start taking notes.")
                }
                .padding(.all, 12)
            }

            Button(action: {
                Task {
                    if case .failure(let e) = await AccountViewModel.updateWelcomeShown(true) {
                        appState.handleError(e)
                    }
                }

                appState.navigate(route: .Folders)
            }) {
                Text("Start Using Mavinote")
                    .frame(maxWidth: .infinity)
                    .foregroundColor(.white)
            }
                .frame(maxWidth: .infinity)
                .padding(12)
                .background(.blue)
                .cornerRadius(8)
                .padding(.all, 12)
        }
        .navigationTitle("Create Folders and Notes")
    }
}
