class AccountViewModel {
    static func accounts() -> AsyncStream<Result<[Account], NoteError>> {
        return Runtime.runStream(DeList<Account>.self) { reax_account_accounts($0) }
    }

    static func account(_ accountId: Int32) async throws -> Account? {
        return try await Runtime.runOnce(DeOption<Account>.self) { reax_account_account($0, accountId) }
    }

    static func devices(_ accountId: Int32) async throws -> [Device] {
        return try await Runtime.runOnce(DeList<Device>.self) { reax_account_devices($0, accountId) }
    }

    static func addDevice(_ accountId: Int32, _ fingerprint: String) async throws -> () {
        return try await Runtime.runOnce(DeUnit.self) { reax_account_add_device($0, accountId, fingerprint) }
    }

    static func removeDevice(_ deviceId: Int32) async throws -> () {
        return try await Runtime.runOnce(DeUnit.self) { reax_account_remove_device($0, deviceId) }
    }

    static func requestVerification(_ email: String) async throws -> String {
        return try await Runtime.runOnce(DeString.self) { reax_account_request_verification($0, email) }
    }

    static func waitVerification(_ token: String) async throws -> () {
        return try await Runtime.runOnce(DeUnit.self) { reax_account_wait_verification($0, token) }
    }

    static func sendVerificationCode(_ email: String) async throws -> () {
        return try await Runtime.runOnce(DeUnit.self) { reax_account_send_verification_code($0, email) }
    }

    static func signUp(_ email: String, _ code: String) async throws -> () {
        return try await Runtime.runOnce(DeUnit.self) { reax_account_sign_up($0, email, code) }
    }

    static func mavinoteAccount(_ accountId: Int32) async throws -> Mavinote? {
        return try await Runtime.runOnce(DeOption<Mavinote>.self) { reax_account_mavinote_account($0, accountId) }
    }

    static func addAccount(_ email: String) async throws -> () {
        return try await Runtime.runOnce(DeUnit.self) { reax_account_add_account($0, email) }
    }

    static func removeAccount(_ accountId: Int32) async throws -> () {
        return try await Runtime.runOnce(DeUnit.self) { reax_account_remove_account($0, accountId) }
    }

    static func sendAccountCloseCode(_ accountId: Int32) async throws -> () {
        return try await Runtime.runOnce(DeUnit.self) { reax_account_send_account_close_code($0, accountId) }
    }

    static func closeAccount(_ accountId: Int32, _ code: String) async throws -> () {
        return try await Runtime.runOnce(DeUnit.self) { reax_account_close_account($0, accountId, code) }
    }

    static func publicKey() async throws -> String {
        return try await Runtime.runOnce(DeString.self) { reax_account_public_key($0) }
    }
}
