typealias AccountResult<T> = Result<T, NoteError>

class AccountViewModel {
    static func accounts() -> AsyncStream<AccountResult<[Account]>> {
        return Runtime.runStream { reax_account_accounts($0) }
    }

    static func account(_ accountId: Int32) async -> AccountResult<Account?> {
        return await Runtime.runOnce { reax_account_account($0, accountId) }
    }

    static func devices(_ accountId: Int32) async -> AccountResult<[Device]> {
        return await Runtime.runOnce { reax_account_devices($0, accountId) }
    }

    static func addDevice(_ accountId: Int32, _ fingerprint: String) async -> AccountResult<()> {
        return await Runtime.runOnceUnit { reax_account_add_device($0, accountId, fingerprint) }
    }

    static func deleteDevice(_ accountId: Int32, _ deviceId: Int32) async -> AccountResult<()> {
        return await Runtime.runOnceUnit { reax_account_delete_device($0, accountId, deviceId) }
    }

    static func requestVerification(_ email: String) async -> AccountResult<String> {
        return await Runtime.runOnce { reax_account_request_verification($0, email) }
    }

    static func waitVerification(_ token: String) async -> AccountResult<()> {
        return await Runtime.runOnceUnit { reax_account_wait_verification($0, token) }
    }

    static func sendVerificationCode(_ email: String) async -> AccountResult<()> {
        return await Runtime.runOnceUnit { reax_account_send_verification_code($0, email) }
    }

    static func signUp(_ email: String, _ code: String) async -> AccountResult<()> {
        return await Runtime.runOnceUnit { reax_account_sign_up($0, email, code) }
    }

    static func mavinoteAccount(_ accountId: Int32) async -> AccountResult<Mavinote?> {
        return await Runtime.runOnce { reax_account_mavinote_account($0, accountId) }
    }

    static func addAccount(_ email: String) async -> AccountResult<()> {
        return await Runtime.runOnceUnit { reax_account_add_account($0, email) }
    }

    static func removeAccount(_ accountId: Int32) async -> AccountResult<()> {
        return await Runtime.runOnceUnit { reax_account_remove_account($0, accountId) }
    }

    static func sendAccountCloseCode(_ accountId: Int32) async -> AccountResult<()> {
        return await Runtime.runOnceUnit { reax_account_send_account_close_code($0, accountId) }
    }

    static func closeAccount(_ accountId: Int32, _ code: String) async -> AccountResult<()> {
        return await Runtime.runOnceUnit { reax_account_close_account($0, accountId, code) }
    }

    static func publicKey() async -> AccountResult<String> {
        return await Runtime.runOnce { reax_account_public_key($0) }
    }

    static func listenNotifications(_ accountId: Int32) -> AsyncStream<AccountResult<DeUnit>> {
        return Runtime.runStream { reax_account_listen_notifications($0, accountId) }
    }

    static func welcomeShown() async -> AccountResult<Bool> {
        return await Runtime.runOnce { reax_account_welcome_shown($0) }
    }

    static func updateWelcomeShown(_ shown: Bool) async -> AccountResult<DeUnit> {
        return await Runtime.runOnce { reax_account_update_welcome_shown($0, shown) }
    }
}
