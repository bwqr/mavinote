use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Token {
    // issued at
    pub iat: i64,
    pub device_id: i32,
}

impl Token {
    pub fn new(device_id: i32) -> Self {
        let now = Utc::now().timestamp();

        Self {
            iat: now,
            device_id,
        }
    }
}
