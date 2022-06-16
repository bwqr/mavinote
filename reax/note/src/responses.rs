use serde::Deserialize;

use crate::models::{Note as NoteModel, NoteState};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub commit_id: i32,
    pub title: Option<String>,
    pub text: String,
}

impl Into<NoteModel> for Note {
    fn into(self) -> NoteModel {
        NoteModel {
            id: self.id,
            folder_id: self.folder_id,
            title: self.title,
            text: self.text,
            commit_id: self.commit_id,
            state: NoteState::Clean,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub commit_id: i32,
    pub note_id: i32,
}
