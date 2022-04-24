use serde::Serialize;

#[derive(Serialize)]
pub struct TokenResponse {
    token: String,
}

impl TokenResponse {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
