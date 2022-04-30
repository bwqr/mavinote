use serde::Deserialize;

use base::sanitize::Sanitize;
use derive::Sanitize;

#[derive(Deserialize, Sanitize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub name: String,
}

#[derive(Deserialize, Sanitize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteRequest {
    #[sanitize(skip_sanitizing)]
    pub folder_id: i32,
    pub title: String,
    pub text: String,
}

#[derive(Deserialize, Sanitize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNoteRequest {
    pub title: String,
    pub text: String,
}
