use std::panic;

use base::{Config, Store};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use wasm_bindgen::prelude::*;

mod log;


#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    log::init();

    runtime::init();
    runtime::put(Store);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    runtime::put(client);

    runtime::put(Config {
        api_url: "http://127.0.0.1:8050/api".to_string(),
        storage_dir: "".to_string(),
    });

    ::log::info!("reax runtime is initialized");

    Ok(())
}

#[wasm_bindgen]
pub async fn folders() -> Result<String, String> {
    note::folders(
        runtime::get::<Store>().unwrap(),
        runtime::get::<Client>().unwrap(),
        runtime::get::<Config>().unwrap(),
    )
    .await
    .map(|f| serde_json::to_string(&f).unwrap())
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn notes(folder_id: i32) -> Result<String, String> {
    note::note_summaries(
        runtime::get::<Store>().unwrap(),
        runtime::get::<Client>().unwrap(),
        runtime::get::<Config>().unwrap(),
        folder_id,
    )
    .await
    .map(|f| serde_json::to_string(&f).unwrap())
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn note(note_id: i32) -> Result<String, String> {
    note::note(
        runtime::get::<Store>().unwrap(),
        runtime::get::<Client>().unwrap(),
        runtime::get::<Config>().unwrap(),
        note_id,
    )
    .await
    .map(|f| serde_json::to_string(&f).unwrap())
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn create_folder(name: String) -> Result<(), String> {
    note::create_folder(
        runtime::get::<Store>().unwrap(),
        runtime::get::<Client>().unwrap(),
        runtime::get::<Config>().unwrap(),
        name,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())?;

    Ok(())
}

#[wasm_bindgen]
pub async fn create_note(folder_id: i32) -> Result<i32, String> {
    note::create_note(
        runtime::get::<Store>().unwrap(),
        runtime::get::<Client>().unwrap(),
        runtime::get::<Config>().unwrap(),
        folder_id,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn update_note(note_id: i32, text: String) -> Result<(), String> {
    note::update_note(
        runtime::get::<Store>().unwrap(),
        runtime::get::<Client>().unwrap(),
        runtime::get::<Config>().unwrap(),
        note_id,
        text,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())
}
