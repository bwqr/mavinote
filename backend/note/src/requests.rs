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
    pub name: String,
    pub text: String,
}

#[derive(Deserialize, Sanitize)]
pub struct UpdateNoteRequest {
    pub commit: i32,
    pub device_notes: Vec<CreateNoteRequest>,
}

#[derive(Deserialize, Sanitize)]
pub struct CreateRequests {
    pub folder_ids: Vec<i32>,
    pub note_ids: Vec<i32>,
}

#[derive(Deserialize)]
pub struct FolderId {
    pub folder_id: i32,
}

#[derive(Deserialize, Sanitize)]
pub struct RespondRequests {
    pub device_id: i32,
    pub folders: Vec<RespondFolderRequest>,
    pub notes: Vec<RespondNoteRequest>,
}

#[derive(Deserialize, Sanitize)]
pub struct RespondFolderRequest {
    pub folder_id: i32,
    pub name: String,
}

#[derive(Deserialize, Sanitize)]
pub struct RespondNoteRequest {
    pub note_id: i32,
    pub name: String,
    pub text: String,
}
