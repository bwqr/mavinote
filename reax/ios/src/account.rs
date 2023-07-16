use std::ffi::{c_char, CStr};

use tokio::task::JoinHandle;

#[no_mangle]
pub extern "C" fn reax_account_accounts(stream_id: i32) -> *mut JoinHandle<()> {
    universal::account::accounts(stream_id)
}

#[no_mangle]
pub extern "C" fn reax_account_account(once_id: i32, account_id: i32) -> *mut JoinHandle<()> {
    universal::account::account(once_id, account_id)
}

#[no_mangle]
pub extern "C" fn reax_account_add_account(
    once_id: i32,
    email: *const c_char,
) -> *mut JoinHandle<()> {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };

    universal::account::add_account(once_id, email)
}

#[no_mangle]
pub extern "C" fn reax_account_add_device(
    once_id: i32,
    account_id: i32,
    fingerprint: *const c_char,
) -> *mut JoinHandle<()> {
    let fingerprint = unsafe { CStr::from_ptr(fingerprint).to_str().unwrap().to_string() };

    universal::account::add_device(once_id, account_id, fingerprint)
}

#[no_mangle]
pub extern "C" fn reax_account_close_account(
    once_id: i32,
    account_id: i32,
    code: *const c_char,
) -> *mut JoinHandle<()> {
    let code = unsafe { CStr::from_ptr(code).to_str().unwrap().to_string() };

    universal::account::close_account(once_id, account_id, code)
}

#[no_mangle]
pub extern "C" fn reax_account_devices(once_id: i32, account_id: i32) -> *mut JoinHandle<()> {
    universal::account::devices(once_id, account_id)
}

#[no_mangle]
pub extern "C" fn reax_account_mavinote_account(
    once_id: i32,
    account_id: i32,
) -> *mut JoinHandle<()> {
    universal::account::mavinote_account(once_id, account_id)
}

#[no_mangle]
pub extern "C" fn reax_account_public_key(once_id: i32) -> *mut JoinHandle<()> {
    universal::account::public_key(once_id)
}

#[no_mangle]
pub extern "C" fn reax_account_remove_account(
    once_id: i32,
    account_id: i32,
) -> *mut JoinHandle<()> {
    universal::account::remove_account(once_id, account_id)
}

#[no_mangle]
pub extern "C" fn reax_account_delete_device(once_id: i32, account_id: i32, device_id: i32) -> *mut JoinHandle<()> {
    universal::account::delete_device(once_id, account_id, device_id)
}

#[no_mangle]
pub extern "C" fn reax_account_request_verification(
    once_id: i32,
    email: *const c_char,
) -> *mut JoinHandle<()> {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };

    universal::account::request_verification(once_id, email)
}

#[no_mangle]
pub extern "C" fn reax_account_send_account_close_code(
    once_id: i32,
    account_id: i32,
) -> *mut JoinHandle<()> {
    universal::account::send_account_close_code(once_id, account_id)
}

#[no_mangle]
pub extern "C" fn reax_account_send_verification_code(
    once_id: i32,
    email: *const c_char,
) -> *mut JoinHandle<()> {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };

    universal::account::send_verification_code(once_id, email)
}

#[no_mangle]
pub extern "C" fn reax_account_sign_up(
    once_id: i32,
    email: *const c_char,
    code: *const c_char,
) -> *mut JoinHandle<()> {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };
    let code = unsafe { CStr::from_ptr(code).to_str().unwrap().to_string() };

    universal::account::sign_up(once_id, email, code)
}

#[no_mangle]
pub extern "C" fn reax_account_wait_verification(
    once_id: i32,
    token: *const c_char,
) -> *mut JoinHandle<()> {
    let token = unsafe { CStr::from_ptr(token).to_str().unwrap().to_string() };

    universal::account::wait_verification(once_id, token)
}

#[no_mangle]
pub extern "C" fn reax_account_listen_notifications(
    stream_id: i32,
    account_id: i32
) -> * mut JoinHandle<()> {
    universal::account::listen_notifications(stream_id, account_id)
}

#[no_mangle]
pub extern "C" fn reax_account_welcome_shown(
    once_id: i32,
) -> * mut JoinHandle<()> {
    universal::account::welcome_shown(once_id)
}

#[no_mangle]
pub extern "C" fn reax_account_update_welcome_shown(
    once_id: i32,
    shown: bool
) -> * mut JoinHandle<()> {
    universal::account::update_welcome_shown(once_id, shown)
}
