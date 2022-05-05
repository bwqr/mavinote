use reqwest::StatusCode;
use serde::Serialize;

pub mod models;

#[derive(Serialize)]
pub enum Error {
    HttpError(HttpError),
    Message(String),
}

#[derive(Serialize)]
pub enum HttpError {
    NoConnection,
    UnexpectedResponse,
    Unauthorized,
    Unknown,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if let Some(StatusCode::UNAUTHORIZED) = e.status() {
            return Error::HttpError(HttpError::Unauthorized)
        }

        Error::HttpError(HttpError::NoConnection)
    }
}

#[derive(Debug)]
pub struct Config {
    pub api_url: String,
    pub storage_dir: String,
}
