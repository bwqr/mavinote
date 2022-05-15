use std::panic;

use base::{Config, Store};
use once_cell::sync::OnceCell;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use wasm_bindgen::prelude::*;

static CONFIG: OnceCell<Config> = OnceCell::new();
static CLIENT: OnceCell<Client> = OnceCell::new();
static STORE: OnceCell<Store> = OnceCell::new();

#[wasm_bindgen]
pub async fn folders() -> Result<String, String> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    note::folders(
        STORE.get_or_init(|| Store),
        CLIENT.get_or_init(|| client),
        CONFIG.get_or_init(|| Config {
            api_url: "http://127.0.0.1:8050/api".to_string(),
            storage_dir: "".to_string(),
        }),
    )
    .await
    .map(|f| serde_json::to_string(&f).unwrap())
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn notes(folder_id: i32) -> Result<String, String> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    note::note_summaries(
        STORE.get_or_init(|| Store),
        CLIENT.get_or_init(|| client),
        CONFIG.get_or_init(|| Config {
            api_url: "http://127.0.0.1:8050/api".to_string(),
            storage_dir: "".to_string(),
        }),
        folder_id,
    )
    .await
    .map(|f| serde_json::to_string(&f).unwrap())
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn note(note_id: i32) -> Result<String, String> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    note::note(
        STORE.get_or_init(|| Store),
        CLIENT.get_or_init(|| client),
        CONFIG.get_or_init(|| Config {
            api_url: "http://127.0.0.1:8050/api".to_string(),
            storage_dir: "".to_string(),
        }),
        note_id,
    )
    .await
    .map(|f| serde_json::to_string(&f).unwrap())
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn create_folder(name: String) -> Result<(), String> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    note::create_folder(
        STORE.get_or_init(|| Store),
        CLIENT.get_or_init(|| client),
        CONFIG.get_or_init(|| Config {
            api_url: "http://127.0.0.1:8050/api".to_string(),
            storage_dir: "".to_string(),
        }),
        name,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())?;

    Ok(())
}

#[wasm_bindgen]
pub async fn create_note(folder_id: i32) -> Result<i32, String> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    note::create_note(
        STORE.get_or_init(|| Store),
        CLIENT.get_or_init(|| client),
        CONFIG.get_or_init(|| Config {
            api_url: "http://127.0.0.1:8050/api".to_string(),
            storage_dir: "".to_string(),
        }),
        folder_id,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn update_note(note_id: i32, text: String) -> Result<(), String> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    note::update_note(
        STORE.get_or_init(|| Store),
        CLIENT.get_or_init(|| client),
        CONFIG.get_or_init(|| Config {
            api_url: "http://127.0.0.1:8050/api".to_string(),
            storage_dir: "".to_string(),
        }),
        note_id,
        text,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())
}
