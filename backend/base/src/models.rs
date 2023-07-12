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
    pub user_id: i32,
    pub device_id: i32,
}

impl Token {
    pub fn device(user_id: i32, device_id: i32) -> Self {
        const TIMEOUT: i64 = 60 * 60 * 24 * 7;

        let now = Utc::now().timestamp();

        Self {
            iat: now,
            exp: now + TIMEOUT,
            kind: TokenKind::Device,
            user_id,
            device_id,
        }
    }

    pub fn pending_device(user_id: i32, device_id: i32) -> Self {
        const TIMEOUT: i64 = 60 * 5;

        let now = Utc::now().timestamp();

        Self {
            iat: now,
            exp: now + TIMEOUT,
            kind: TokenKind::PendingDevice,
            user_id,
            device_id,
        }
    }
}
