import SwiftUI

struct LoginView : View {
    @EnvironmentObject var appState: AppState
    @State var inProgress = false
    @State var email = ""
    @State var password = ""
    
    var body: some View {
        VStack {
            TextField("Email", text: $email)
                .keyboardType(.emailAddress)
            SecureField("Password", text: $password)
            Button("Login") {
                if (inProgress) {
                    return
                }
                
                inProgress = true
                
                Task {
                    do {
                        try await AuthViewModel().login(email, password)
                        appState.navigate(Screen.BackgroundFeatures)
                    } catch {
                        print("failed to login", error)
                    }
                    
                    inProgress = false
                }
            }
        }
        .navigationTitle("Login")
        .navigationBarBackButtonHidden(true)
    }
}
