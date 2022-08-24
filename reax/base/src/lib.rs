use reqwest::StatusCode;
use serde::Serialize;

pub mod models;
pub mod observable_map;

#[derive(Clone, Debug)]
pub enum State<T, E> {
    Ok(T),
    Err(E),
    Loading,
    Initial,
}

impl<T, E> Default for State<T, E> {
    fn default() -> Self {
        Self::Initial
    }
}

impl<T, E> From<Result<T, E>> for State<T, E> {
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(ok) => Self::Ok(ok),
            Err(e) => Self::Err(e),
        }
    }
}

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
    Unauthorized(Option<i32>),
    Unknown,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if let Some(StatusCode::UNAUTHORIZED) = e.status() {
            return Error::Http(HttpError::Unauthorized(None))
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

#[cfg(feature = "sqlx")]
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
