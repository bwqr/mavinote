use serde::Deserialize;

use crate::models::{State, RemoteId};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub state: State,
}

impl Folder {
    pub fn id(&self) -> RemoteId {
        RemoteId(self.id)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub title: Option<String>,
    pub text: String,
    pub commit_id: i32,
    pub state: State,
}

impl Note {
    pub fn id(&self) -> RemoteId {
        RemoteId(self.id)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub commit_id: i32,
    pub note_id: i32,
    pub state: State,
}

impl Commit {
    pub fn note_id(&self) -> RemoteId {
        RemoteId(self.note_id)
    }
}
