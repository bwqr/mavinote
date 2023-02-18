use base::sanitize::Sanitize;
use derive::Sanitize;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub pubkey: String,
    pub password: String,
}

#[derive(Deserialize, Sanitize)]
pub struct SignUp {
    pub email: String,
    pub code: String,
    pub pubkey: String,
    pub password: String,
}

#[derive(Deserialize, Sanitize)]
pub struct SendCode {
    pub email: String,
}

#[derive(Deserialize, Sanitize)]
pub struct CreatePendingDevice {
    pub email: String,
    pub pubkey: String,
    pub password: String,
}
