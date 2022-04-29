use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct Folder {
    pub id: i32,
    pub name: String,
}

#[derive(FromRow, Serialize)]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub title: String,
    pub text: String,
}
