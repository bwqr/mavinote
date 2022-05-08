use std::{os::raw::{c_int, c_char}, ffi::CStr};

use crate::send;

#[no_mangle]
pub extern fn reax_auth_login(wait_id: c_int, email: * const c_char, password: * const c_char) {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };
    let password = unsafe { CStr::from_ptr(password).to_str().unwrap().to_string() };

    runtime::spawn(async move {
        let res = auth::login(runtime::store(), runtime::client(), runtime::config(), email, password).await;

        send(wait_id, res);
    });
}
