use std::ffi::{c_char, CStr, c_void};

#[no_mangle]
pub extern "C" fn reax_account_accounts(stream_id: i32) -> * mut c_void {
    universal::account::accounts(stream_id) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_account(once_id: i32, account_id: i32) -> * mut c_void {
    universal::account::account(once_id, account_id) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_add_account(
    once_id: i32,
    email: *const c_char,
) -> * mut c_void {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };

    universal::account::add_account(once_id, email) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_add_device(
    once_id: i32,
    account_id: i32,
    fingerprint: *const c_char,
) -> * mut c_void {
    let fingerprint = unsafe { CStr::from_ptr(fingerprint).to_str().unwrap().to_string() };

    universal::account::add_device(once_id, account_id, fingerprint) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_close_account(
    once_id: i32,
    account_id: i32,
    code: *const c_char,
) -> * mut c_void {
    let code = unsafe { CStr::from_ptr(code).to_str().unwrap().to_string() };

    universal::account::close_account(once_id, account_id, code) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_devices(once_id: i32, account_id: i32) -> * mut c_void {
    universal::account::devices(once_id, account_id) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_mavinote_account(
    once_id: i32,
    account_id: i32,
) -> * mut c_void {
    universal::account::mavinote_account(once_id, account_id) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_public_key(once_id: i32) -> * mut c_void {
    universal::account::public_key(once_id) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_remove_account(
    once_id: i32,
    account_id: i32,
) -> * mut c_void {
    universal::account::remove_account(once_id, account_id) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_delete_device(once_id: i32, account_id: i32, device_id: i32) -> * mut c_void {
    universal::account::delete_device(once_id, account_id, device_id) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_request_verification(
    once_id: i32,
    email: *const c_char,
) -> * mut c_void {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };

    universal::account::request_verification(once_id, email) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_send_account_close_code(
    once_id: i32,
    account_id: i32,
) -> * mut c_void {
    universal::account::send_account_close_code(once_id, account_id) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_send_verification_code(
    once_id: i32,
    email: *const c_char,
) -> * mut c_void {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };

    universal::account::send_verification_code(once_id, email) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_sign_up(
    once_id: i32,
    email: *const c_char,
    code: *const c_char,
) -> * mut c_void {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };
    let code = unsafe { CStr::from_ptr(code).to_str().unwrap().to_string() };

    universal::account::sign_up(once_id, email, code) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_wait_verification(
    once_id: i32,
    token: *const c_char,
) -> * mut c_void {
    let token = unsafe { CStr::from_ptr(token).to_str().unwrap().to_string() };

    universal::account::wait_verification(once_id, token) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_listen_notifications(
    stream_id: i32,
    account_id: i32
) -> * mut c_void {
    universal::account::listen_notifications(stream_id, account_id) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_welcome_shown(once_id: i32) -> * mut c_void {
    universal::account::welcome_shown(once_id) as * mut c_void
}

#[no_mangle]
pub extern "C" fn reax_account_update_welcome_shown(once_id: i32, shown: bool) -> * mut c_void {
    universal::account::update_welcome_shown(once_id, shown) as * mut c_void
}
