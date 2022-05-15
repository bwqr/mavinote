use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
};

use base::{Store, Config};
use reqwest::Client;

use crate::send;

#[no_mangle]
pub extern "C" fn reax_auth_login(wait_id: c_int, email: *const c_char, password: *const c_char) {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };
    let password = unsafe { CStr::from_ptr(password).to_str().unwrap().to_string() };

    crate::spawn(async move {
        let res = auth::login(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            email,
            password,
        )
        .await;

        send(wait_id, res);
    });
}
