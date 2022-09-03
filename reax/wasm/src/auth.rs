use ::note::accounts::mavinote::MavinoteClient;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::{serialize_to_buffer, setItem};

#[wasm_bindgen]
pub async fn auth_login(email: String, password: String) -> Result<(), Uint8Array> {
    let mavinote = runtime::get::<MavinoteClient>().unwrap();

    let token = mavinote.login(
        email.as_str(),
        password.as_str(),
    )
    .await
    .map_err(serialize_to_buffer)?;

    setItem("token", &token.token);

    runtime::put::<MavinoteClient>(mavinote.with_token(token.token));

    Ok(())
}
