use serde::{Serialize, Deserialize};
use sqlx::FromRow;

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
    pub title: String,
    pub text: String,
}
