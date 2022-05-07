use std::{os::raw::{c_char, c_int, c_uchar}, ffi::{CStr, c_void}, sync::{Mutex, mpsc::Sender}};

use once_cell::sync::OnceCell;
use serde::Serialize;

mod note;

static HANDLER: OnceCell<Mutex<Sender<(i32, bool, Vec<u8>)>>> = OnceCell::new();

pub(crate) fn send<T: Serialize, E: Serialize>(wait_id: i32, res: Result<T, E>) {
    let bytes = match &res {
        Ok(value) => bincode::serialize(value).expect("failed to serialize val"),
        Err(e) => bincode::serialize(e).expect("failed to serialize val"),
    };

    HANDLER.get().unwrap().lock().unwrap().send((wait_id, res.is_ok(), bytes)).unwrap();
}

#[no_mangle]
pub extern fn reax_init(api_url: *const c_char, storage_dir: *const c_char) {
    let api_url = unsafe { CStr::from_ptr(api_url).to_str().unwrap().to_string() };
    let storage_dir = unsafe { CStr::from_ptr(storage_dir).to_str().unwrap().to_string() };

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    runtime::init(base::Config { api_url, storage_dir });

    log::info!("reax runtime is initialized");
}

#[no_mangle]
pub extern fn reax_init_handler(ptr: *const c_void, f: unsafe extern fn(c_int, c_uchar, *const c_uchar, c_int, *const c_void)) {
    let (send, recv) = std::sync::mpsc::channel();

    HANDLER
        .set(Mutex::new(send))
        .map_err(|_| "HandlerError")
        .expect("failed to set handler");

    while let Ok((wait_id, ok, bytes)) = recv.recv() {
        unsafe { f(wait_id, ok as c_uchar, bytes.as_ptr() as *const c_uchar, bytes.len() as c_int, ptr) }
    }
}
