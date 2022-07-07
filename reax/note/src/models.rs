use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Type};

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub state: State,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub title: Option<String>,
    pub text: String,
    pub commit_id: i32,
    pub state: State,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Type)]
pub enum State {
    Clean,
    Modified,
    Deleted,
}
