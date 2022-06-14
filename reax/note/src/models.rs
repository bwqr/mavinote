use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Type};

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub title: Option<String>,
    pub text: String,
    pub commit_id: i32,
    pub state: NoteState,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
pub enum NoteState {
    Clean,
    Modified,
    Deleted,
}
