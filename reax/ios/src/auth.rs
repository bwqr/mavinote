use std::{
    ffi::CStr,
    os::raw::{c_char, c_int}, sync::Arc,
};

use base::{Store, Config};
use reqwest::Client;
use tokio::task::JoinHandle;

use crate::send_once;

#[no_mangle]
pub extern "C" fn reax_auth_login(once_id: c_int, email: *const c_char, password: *const c_char) -> * mut JoinHandle<()> {
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };
    let password = unsafe { CStr::from_ptr(password).to_str().unwrap().to_string() };

    let handle = crate::spawn(async move {
        let res = auth::login(
            runtime::get::<Arc<dyn Store>>().unwrap(),
            runtime::get::<Arc<Client>>().unwrap(),
            runtime::get::<Arc<Config>>().unwrap(),
            email,
            password,
        )
        .await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}
