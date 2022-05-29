class AuthViewModel {
    func login(_ email: String, _ password: String) async throws -> () {
        return try await withCheckedThrowingContinuation { continuation in
            Runtime.instance().startOnce(Once(
                onNext: { deserializer in continuation.resume(returning: ())},
                onError: { continuation.resume(throwing: $0)},
                onStart: { reax_auth_login($0, email, password)}
            ))
        }
    }
}
