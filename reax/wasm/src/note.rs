use std::sync::Arc;

use base::{Store, Config};
use reqwest::Client;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn note_folders() -> Result<String, String> {
    note::folders(
        runtime::get::<Arc<dyn Store>>().unwrap(),
        runtime::get::<Arc<Client>>().unwrap(),
        runtime::get::<Arc<Config>>().unwrap(),
    )
    .await
    .map(|f| serde_json::to_string(&f).unwrap())
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn note_notes(folder_id: i32) -> Result<String, String> {
    note::note_summaries(
        runtime::get::<Arc<dyn Store>>().unwrap(),
        runtime::get::<Arc<Client>>().unwrap(),
        runtime::get::<Arc<Config>>().unwrap(),
        folder_id,
    )
    .await
    .map(|f| serde_json::to_string(&f).unwrap())
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn note_note(note_id: i32) -> Result<String, String> {
    note::note(
        runtime::get::<Arc<dyn Store>>().unwrap(),
        runtime::get::<Arc<Client>>().unwrap(),
        runtime::get::<Arc<Config>>().unwrap(),
        note_id,
    )
    .await
    .map(|f| serde_json::to_string(&f).unwrap())
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn note_create_folder(name: String) -> Result<(), String> {
    note::create_folder(
        runtime::get::<Arc<dyn Store>>().unwrap(),
        runtime::get::<Arc<Client>>().unwrap(),
        runtime::get::<Arc<Config>>().unwrap(),
        name,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())?;

    Ok(())
}

#[wasm_bindgen]
pub async fn note_create_note(folder_id: i32) -> Result<i32, String> {
    note::create_note(
        runtime::get::<Arc<dyn Store>>().unwrap(),
        runtime::get::<Arc<Client>>().unwrap(),
        runtime::get::<Arc<Config>>().unwrap(),
        folder_id,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())
}

#[wasm_bindgen]
pub async fn note_update_note(note_id: i32, text: String) -> Result<(), String> {
    note::update_note(
        runtime::get::<Arc<dyn Store>>().unwrap(),
        runtime::get::<Arc<Client>>().unwrap(),
        runtime::get::<Arc<Config>>().unwrap(),
        note_id,
        text,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())
}
