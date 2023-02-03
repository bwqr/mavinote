use std::fmt::Display;

use actix_web::{error::BlockingError, http::StatusCode, HttpResponse, ResponseError};
use serde::{ser::SerializeStruct, Serialize};

pub mod crypto;
pub mod middlewares;
pub mod models;
pub mod sanitize;
pub mod schema;
pub mod types;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct HttpMessage {
    message: &'static str,
}

impl HttpMessage {
    pub const fn success() -> HttpMessage {
        HttpMessage { message: "success" }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct HttpError {
    pub code: StatusCode,
    pub error: &'static str,
    pub message: Option<String>,
}

impl HttpError {
    pub const fn not_found(error: &'static str) -> Self {
        HttpError {
            code: StatusCode::NOT_FOUND,
            error,
            message: None,
        }
    }

    pub const fn conflict(error: &'static str) -> Self {
        HttpError {
            code: StatusCode::CONFLICT,
            error,
            message: None,
        }
    }

    pub const fn unprocessable_entity(error: &'static str) -> Self {
        HttpError {
            code: StatusCode::UNPROCESSABLE_ENTITY,
            error,
            message: None,
        }
    }
}

impl Serialize for HttpError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("HttpError", 3)?;
        state.serialize_field("code", &StatusCode::as_u16(&self.code))?;
        state.serialize_field("error", &self.error)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "code: {}, error: {}, message: {}",
            self.code,
            self.error,
            self.message.as_ref().map(|s| s.as_str()).unwrap_or("null")
        )
    }
}

impl ResponseError for HttpError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.code
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.code).json(self)
    }
}

impl From<BlockingError> for HttpError {
    fn from(_: BlockingError) -> Self {
        HttpError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            error: "blocking_error",
            message: None,
        }
    }
}

impl From<diesel::result::Error> for HttpError {
    fn from(e: diesel::result::Error) -> Self {
        log::error!("db error, {:?}", e);

        if let diesel::result::Error::NotFound = e {
            return HttpError {
                code: StatusCode::NOT_FOUND,
                error: "item_not_found",
                message: None,
            };
        };

        HttpError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            error: "db_error",
            message: None,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for HttpError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        HttpError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            error: "crypto_error",
            message: None,
        }
    }
}
