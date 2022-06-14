use serde::Serialize;

use crate::models::{Note as NoteModel, Commit as CommitModel};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub commit_id: i32,
    pub title: Option<String>,
    pub text: String,
}

impl From<(NoteModel, CommitModel)> for Note {
    fn from(models: (NoteModel, CommitModel)) -> Self {
        Note {
            id: models.0.id,
            folder_id: models.0.folder_id,
            commit_id: models.1.id,
            title: models.0.title,
            text: models.1.text,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub commit_id: i32,
    pub note_id: i32,
}
