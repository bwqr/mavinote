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
    pub title: Option<String>,
    pub text: String,
}
