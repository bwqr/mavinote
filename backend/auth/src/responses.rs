use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub token: String,
}

