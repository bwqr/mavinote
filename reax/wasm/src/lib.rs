use std::{panic, sync::Arc};

use base::{Config, Store};

use reqwest::{
    header::{HeaderMap, HeaderValue}, ClientBuilder,
};
use wasm_bindgen::prelude::*;

mod log;
pub mod auth;
pub mod note;

pub struct LocalStorage;

impl Store for LocalStorage {
    fn get<'a>(&'a self, key: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<String>, base::Error>> + Send + 'a>> {
        Box::pin(async move {
            Ok(getItem(key))
        })
    }

    fn put<'a>(&'a self, key: &'a str, value: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), base::Error>> + Send + 'a>> {
        Box::pin(async move {
            setItem(key, value);
            Ok(())
        })
    }
}


#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    log::init();

    runtime::init();
    runtime::put::<Arc<dyn Store>>(Arc::new(LocalStorage));

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    runtime::put(Arc::new(client));

    runtime::put(Arc::new(Config {
        api_url: "http://127.0.0.1:8050/api".to_string(),
        storage_dir: "".to_string(),
    }));

    ::log::info!("reax runtime is initialized");

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localStorage)]
    fn getItem(key: &str) -> Option<String>;

    #[wasm_bindgen(js_namespace = localStorage)]
    fn setItem(key: &str, value: &str);
}

#[wasm_bindgen]
pub fn set_local(key: String, value: String) {
    setItem(key.as_str(), value.as_str());
}
