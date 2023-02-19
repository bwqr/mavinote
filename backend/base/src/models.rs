use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::HttpError;

pub const UNEXPECTED_TOKEN_KIND: HttpError = HttpError {
    code: actix_web::http::StatusCode::UNAUTHORIZED,
    error: "unexpected_token_kind",
    message: None,
};

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub enum TokenKind {
    Device,
    PendingDevice,
}

#[derive(Deserialize, Serialize)]
pub struct Token {
    // issued at
    pub iat: i64,
    // expire time
    pub exp: i64,
    pub kind: TokenKind,
    pub id: i32,
}

impl Token {
    pub fn device(device_id: i32) -> Self {
        const TIMEOUT: i64 = 60 * 60 * 24 * 7;

        let now = Utc::now().timestamp();

        Self {
            iat: now,
            exp: now + TIMEOUT,
            kind: TokenKind::Device,
            id: device_id,
        }
    }

    pub fn pending_device(pending_device_id: i32) -> Self {
        const TIMEOUT: i64 = 60 * 5;

        let now = Utc::now().timestamp();

        Self {
            iat: now,
            exp: now + TIMEOUT,
            kind: TokenKind::PendingDevice,
            id: pending_device_id,
        }
    }
}
