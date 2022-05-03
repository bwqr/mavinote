use reqwest::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub enum Error {
    HttpError(HttpError),
}

#[derive(Serialize)]
pub enum HttpError {
    NoConnection,
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
