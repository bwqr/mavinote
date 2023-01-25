use base::sanitize::Sanitize;
use derive::Sanitize;

use serde::Deserialize;

#[derive(Deserialize, Sanitize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Sanitize)]
pub struct SignUp {
    pub email: String,
    pub code: String,
}

#[derive(Deserialize, Sanitize)]
pub struct SendCode {
    pub email: String,
}
