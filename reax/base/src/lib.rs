use std::{ops::Deref, sync::Arc};

use reqwest::StatusCode;
use serde::Serialize;

pub mod models;
mod store;

pub use store::Store;

pub struct Data<T: ?Sized>(Arc<T>);

impl<T> Data<T> {
    pub fn new(value: T) -> Self {
        Data(Arc::new(value))
    }

    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Data<T> {
    pub fn from_arc(value: Arc<T>) -> Self {
        Data(value)
    }
}

impl<T: ?Sized> Deref for Data<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for Data<T> {
    fn clone(&self) -> Self {
        Data(Arc::clone(&self.0))
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    Http(HttpError),
    Message(String),
}

#[derive(Clone, Debug, Serialize)]
pub enum HttpError {
    NoConnection,
    UnexpectedResponse,
    Unauthorized,
    Unknown,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if let Some(StatusCode::UNAUTHORIZED) = e.status() {
            return Error::Http(HttpError::Unauthorized)
        }

        #[cfg(not(target_arch = "wasm32"))]
        if e.is_connect() {
            return Error::Http(HttpError::NoConnection)
        }

        if e.is_decode() {
            return Error::Http(HttpError::UnexpectedResponse)
        }

        Error::Http(HttpError::Unknown)
    }
}

#[derive(Debug)]
pub struct Config {
    pub api_url: String,
    pub storage_dir: String,
}
