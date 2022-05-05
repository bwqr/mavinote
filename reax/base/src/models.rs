use serde::Deserialize;

#[derive(Deserialize)]
pub struct Token {
    pub token: String,
}
