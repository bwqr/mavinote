use serde::Deserialize;

use base::sanitize::Sanitize;
use derive::Sanitize;

#[derive(Deserialize, Sanitize)]
pub struct CreateFolderRequest {
    pub name: String,
    pub device_id: i32,
}

#[derive(Deserialize, Sanitize)]
pub struct CreateNoteRequest {
    pub device_id: i32,
    pub title: Option<String>,
    pub text: String,
}

#[derive(Deserialize, Sanitize)]
pub struct UpdateNoteRequest {
    pub commit: i32,
    pub device_notes: Vec<CreateNoteRequest>,
}
