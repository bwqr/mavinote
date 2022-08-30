use account::{Account, Mavinote};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::serialize_to_buffer;

#[wasm_bindgen]
pub async fn note_folders() -> Result<Uint8Array, Uint8Array> {
    let mavinote = runtime::get::<Mavinote>().unwrap();

    mavinote.fetch_folders().await
        .map(serialize_to_buffer)
        .map_err(serialize_to_buffer)
}
