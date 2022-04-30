use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub name: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteRequest {
    pub folder_id: i32,
    pub title: String,
    pub text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNoteRequest {
    pub title: String,
    pub text: String,
}
