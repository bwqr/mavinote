use base::{Store, Config, Data};
use reqwest::Client;
use wasm_bindgen::prelude::*;

use crate::LocalStorage;

#[wasm_bindgen]
pub async fn auth_login(email: String, password: String) -> Result<(), String> {
    auth::login(
        Data::from_arc(runtime::get::<LocalStorage>().unwrap().into_inner()),
        runtime::get::<Client>().unwrap(),
        runtime::get::<Config>().unwrap(),
        email,
        password,
    )
    .await
    .map_err(|e| serde_json::to_string(&e).unwrap())?;

    Ok(())
}
