use std::{os::raw::{c_char, c_int, c_uchar}, ffi::{CStr, c_void}, sync::{Mutex, mpsc::Sender}, future::Future};

use base::{Store, Config};
use once_cell::sync::OnceCell;
use reqwest::{header::{HeaderMap, HeaderValue}, ClientBuilder, Client};
use serde::Serialize;
use tokio::task::JoinHandle;

mod note;
mod auth;

static ASYNC_RUNTIME: OnceCell<tokio::runtime::Runtime> = OnceCell::new();
static HANDLER: OnceCell<Mutex<Sender<(i32, bool, Vec<u8>)>>> = OnceCell::new();

pub(crate) fn send<T: Serialize, E: Serialize>(wait_id: i32, res: Result<T, E>) {
    let bytes = match &res {
        Ok(value) => bincode::serialize(value).expect("failed to serialize val"),
        Err(e) => bincode::serialize(e).expect("failed to serialize val"),
    };

    HANDLER.get().unwrap().lock().unwrap().send((wait_id, res.is_ok(), bytes)).unwrap();
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    ASYNC_RUNTIME.get().unwrap().spawn(future)
}

#[no_mangle]
pub extern fn reax_init(api_url: *const c_char, storage_dir: *const c_char) {
    let api_url = unsafe { CStr::from_ptr(api_url).to_str().unwrap().to_string() };
    let storage_dir = unsafe { CStr::from_ptr(storage_dir).to_str().unwrap().to_string() };

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    ASYNC_RUNTIME
        .set(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to initialize tokio runtime"),
        )
        .expect("failed to set tokio runtime");

    runtime::init();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    runtime::put::<Client>(client);

    runtime::put::<Store>(Store);

    runtime::put::<Config>(Config {
        api_url,
        storage_dir,
    });

    ::log::info!("reax runtime is initialized");
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
