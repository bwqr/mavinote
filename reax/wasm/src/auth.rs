use account::Mavinote;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::{serialize_to_buffer, setItem};

#[wasm_bindgen]
pub async fn auth_login(email: String, password: String) -> Result<(), Uint8Array> {
    let mavinote = runtime::get::<Mavinote>().unwrap();

    let token = mavinote.login(
        email,
        password,
    )
    .await
    .map_err(serialize_to_buffer)?;

    setItem("token", &token.token);

    runtime::put::<Mavinote>(mavinote.with_token(token.token));

    Ok(())
}
