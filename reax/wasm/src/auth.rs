use std::sync::Arc;

use base::{Store, Config};
use reqwest::Client;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn auth_login(email: String, password: String) -> Result<(), String> {
    auth::login(
        runtime::get::<Arc<dyn Store>>().unwrap(),
        runtime::get::<Arc<Client>>().unwrap(),
        runtime::get::<Arc<Config>>().unwrap(),
        email,
        password,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())?;

    Ok(())
}
