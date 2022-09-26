use std::sync::Arc;

use base::Config;
use ::note::accounts::mavinote::MavinoteClient;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::{serialize_to_buffer, setItem, removeItem};

#[wasm_bindgen]
pub async fn auth_login(email: String, password: String) -> Result<(), Uint8Array> {
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = MavinoteClient::login(
        config.api_url.as_str(),
        email.as_str(),
        password.as_str(),
    )
    .await
    .map_err(serialize_to_buffer)?;

    setItem("token", &token.token);

    let mavinote = runtime::get::<Arc<MavinoteClient>>().unwrap();
    runtime::put::<Arc<MavinoteClient>>(Arc::new(mavinote.with_token(token.token)));

    Ok(())
}

#[wasm_bindgen]
pub async fn auth_sign_up(name: String, email: String, password: String) -> Result<(), Uint8Array> {
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = MavinoteClient::sign_up(
        config.api_url.as_str(),
        name.as_str(),
        email.as_str(),
        password.as_str(),
    )
    .await
    .map_err(serialize_to_buffer)?;

    setItem("token", &token.token);

    let mavinote = runtime::get::<Arc<MavinoteClient>>().unwrap();
    runtime::put::<Arc<MavinoteClient>>(Arc::new(mavinote.with_token(token.token)));

    Ok(())
}

#[wasm_bindgen]
pub async fn auth_logout() -> () {
    removeItem("token");
}
