use serde::Deserialize;

use base::sanitize::Sanitize;
use derive::Sanitize;

#[derive(Sanitize, Deserialize)]
pub struct AddDevice {
    pub pubkey: String,
}

#[derive(Deserialize)]
pub struct DeleteDevice {
    pub id: Option<i32>,
}

#[derive(Sanitize, Deserialize)]
pub struct CloseAccount {
    pub code: String,
}
