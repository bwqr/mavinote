use reqwest::StatusCode;
use serde::Serialize;

pub mod models;
mod store;

pub use store::Store;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    Http(HttpError),
    Message(String),
    Database(String),
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

        if e.is_connect() {
            return Error::Http(HttpError::NoConnection)
        }

        if e.is_decode() {
            return Error::Http(HttpError::UnexpectedResponse)
        }

        Error::Http(HttpError::Unknown)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Database(e.to_string())
    }
}

#[derive(Debug)]
pub struct Config {
    pub api_url: String,
    pub storage_dir: String,
}
