use serde::Deserialize;

#[derive(Deserialize)]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub commit_id: i32,
    pub title: Option<String>,
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub commit_id: i32,
    pub note_id: i32,
}
