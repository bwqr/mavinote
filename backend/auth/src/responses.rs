use serde::Serialize;

#[derive(Serialize)]
pub struct Token {
    token: String,
}

impl Token {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
