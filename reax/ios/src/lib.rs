use std::{os::raw::c_char, ffi::{CStr, c_void}};

mod note;
mod account;

pub(crate) type DeserializeHandler = unsafe extern "C" fn(* const u8, usize) -> * mut c_void;

#[no_mangle]
pub extern fn reax_init(
    api_url: *const c_char,
    ws_url: *const c_char,
    storage_dir: *const c_char,
    f: DeserializeHandler,
) -> * mut c_void {
    let api_url = unsafe { CStr::from_ptr(api_url).to_str().unwrap().to_string() };
    let ws_url = unsafe { CStr::from_ptr(ws_url).to_str().unwrap().to_string() };
    let storage_dir = unsafe { CStr::from_ptr(storage_dir).to_str().unwrap().to_string() };

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let bytes = universal::init(api_url, ws_url, storage_dir.clone());
    unsafe { f(bytes.as_ptr(), bytes.len()) }
}

#[no_mangle]
pub extern fn reax_init_handler(ptr: * mut c_void, f: unsafe extern fn(i32, *const u8, usize, * mut c_void)) {
    let (send, recv) = std::sync::mpsc::channel();

    universal::init_handler(send);

    while let Ok((wait_id, bytes)) = recv.recv() {
        unsafe { f(wait_id, bytes.as_ptr(), bytes.len(), ptr) }
    }
}

#[no_mangle]
pub extern "C" fn reax_abort(pointer: * mut c_void) {
    unsafe { universal::abort(pointer as _); }
}
