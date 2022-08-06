use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest<'a> {
    pub name: &'a str
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteRequest<'a> {
    pub folder_id: i32,
    pub title: Option<&'a str>,
    pub text: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNoteRequest<'a> {
    pub title: Option<&'a str>,
    pub text: &'a str,
}
