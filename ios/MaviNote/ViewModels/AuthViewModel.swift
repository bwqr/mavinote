class AuthViewModel {
    func login(_ email: String, _ password: String) async throws -> () {
        return try await withCheckedThrowingContinuation { continuation in
            let waitId = Runtime.instance().wait(resume: AsyncWait(continuation) { deserializer in
            })

            reax_auth_login(waitId, email, password)
        }
    }
}
