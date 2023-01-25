use serde::Deserialize;

use base::sanitize::Sanitize;
use derive::Sanitize;

#[derive(Sanitize, Deserialize)]
pub struct CreateDeviceRequest {
    pub id: String,
}
