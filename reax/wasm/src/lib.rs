use std::panic;

use ::note::accounts::mavinote::MavinoteClient;
use futures::stream::AbortHandle;
use js_sys::Uint8Array;
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod log;
pub mod auth;
pub mod note;

#[wasm_bindgen]
extern "C" {
    type WasmRuntime;

    #[wasm_bindgen(static_method_of = WasmRuntime)]
    fn handleStream(stream_id: u32, bytes: Vec<u8>);

    #[wasm_bindgen(js_namespace = localStorage)]
    pub(crate) fn getItem(key: &str) -> Option<String>;

    #[wasm_bindgen(js_namespace = localStorage)]
    pub(crate) fn removeItem(key: &str);

    #[wasm_bindgen(js_namespace = localStorage)]
    pub(crate) fn setItem(key: &str, value: &str);
}

#[wasm_bindgen]
pub fn init_wasm(api_url: String) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    log::init();

    runtime::init();
    runtime::put::<MavinoteClient>(MavinoteClient::new(None, api_url, getItem("token").unwrap_or("".to_string())));

    note::init();

    ::log::info!("reax runtime is initialized");
}

#[wasm_bindgen]
pub fn abort_stream(pointer: u32) {
    let handle = unsafe { Box::from_raw(pointer as *mut AbortHandle) };

    ::log::debug!("received abort, {:p}", handle);

    handle.abort();
}

#[derive(Serialize)]
pub(crate) enum Message<T: Serialize> {
    Ok(T),
    Err(base::Error),
    Complete,
}

pub(crate) fn send_stream<T: Serialize>(stream_id: u32, value: Message<T>) {
    let bytes = bincode::serialize(&value).unwrap();

    WasmRuntime::handleStream(stream_id, bytes);
}

pub(crate) fn serialize_to_buffer<T: Serialize>(value: T) -> Uint8Array {
    let bytes = bincode::serialize(&value).unwrap();
    let array = Uint8Array::new_with_length(bytes.len() as u32);
    array.copy_from(&bytes);

    array
}
