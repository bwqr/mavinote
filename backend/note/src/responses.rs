use diesel::Queryable;
use serde::Serialize;

use crate::models::State;

#[derive(Serialize)]
pub struct CreatedFolder {
    pub id: i32,
}

#[derive(Serialize)]
pub struct CreatedNote {
    pub id: i32,
    pub commit: i32,
}

#[derive(Queryable, Serialize)]
pub struct Commit {
    pub note_id: i32,
    pub commit: i32,
    pub state: State,
}

#[derive(Queryable, Serialize)]
pub struct Folder {
    pub id: i32,
    pub state: State,
    pub device_folder: Option<DeviceFolder>,
}

#[derive(Queryable, Serialize)]
pub struct DeviceFolder {
    pub sender_device_id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize)]
pub struct Note {
    pub id: i32,
    pub commit: i32,
    pub state: State,
    pub device_note: Option<DeviceNote>,
}

#[derive(Queryable, Serialize)]
pub struct DeviceNote {
    pub sender_device_id: i32,
    pub name: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct Requests {
    pub folder_requests: Vec<FolderRequest>,
    pub note_requests: Vec<NoteRequest>,
}

#[derive(Queryable, Serialize)]
pub struct FolderRequest {
    pub folder_id: i32,
    pub device_id: i32,
}

#[derive(Queryable, Serialize)]
pub struct NoteRequest {
    pub note_id: i32,
    pub device_id: i32,
}
