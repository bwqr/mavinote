use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Token {
    // issued at
    pub iat: i64,
    // expire time
    pub exp: i64,
    pub device_id: i32,
}

impl Token {
    pub fn new(device_id: i32) -> Self {
        const TIMEOUT: i64 = 60 * 60 * 24 * 7;

        let now = Utc::now().timestamp();

        Self {
            iat: now,
            exp: now + TIMEOUT,
            device_id,
        }
    }
}
