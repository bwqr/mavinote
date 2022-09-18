use serde::Deserialize;

use base::sanitize::Sanitize;
use derive::Sanitize;

#[derive(Deserialize, Sanitize)]
pub struct CreateFolderRequest {
    pub name: String,
}

#[derive(Deserialize, Sanitize)]
pub struct CreateNoteRequest {
    pub folder_id: i32,
    pub title: Option<String>,
    pub text: String,
}

#[derive(Deserialize, Sanitize)]
pub struct UpdateNoteRequest {
    pub commit: i32,
    pub title: Option<String>,
    pub text: String,
}
