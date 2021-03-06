use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Token {
    // issued at
    pub iat: i64,
    // expire time
    pub exp: i64,
    pub user_id: i32,
}

impl Token {
    pub fn new(user_id: i32) -> Self {
        const TIMEOUT: i64 = 60 * 60 * 24;

        let now = Utc::now().timestamp();

        Self {
            iat: now,
            exp: now + TIMEOUT,
            user_id,
        }
    }
}
