use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}