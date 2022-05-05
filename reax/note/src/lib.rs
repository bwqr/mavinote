pub mod models;

use requests::{CreateFolderRequest, CreateNoteRequest, UpdateNoteRequest};
use reqwest::{Client, StatusCode};

use base::{Config, Error};

use models::{Folder, Note};

mod requests;

pub async fn folders(
    client: &'static Client,
    config: &'static Config,
) -> Result<Vec<Folder>, Error> {
    client
        .get(format!("{}/note/folders", config.api_url))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn create_folder(
    client: &'static Client,
    config: &'static Config,
    name: String,
) -> Result<(), Error> {
    client
        .post(format!("{}/note/folder", config.api_url))
        .body(serde_json::to_string(&CreateFolderRequest { name }).unwrap())
        .send()
        .await?
        .error_for_status()
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn note_summaries(
    client: &'static Client,
    config: &'static Config,
    folder_id: i32,
) -> Result<Vec<Note>, Error> {
    client
        .get(format!(
            "{}/note/folder/{}/notes",
            config.api_url, folder_id
        ))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn note(
    client: &'static Client,
    config: &'static Config,
    note_id: i32,
) -> Result<Option<Note>, Error> {
    let response = client
        .get(format!("{}/note/note/{}", config.api_url, note_id))
        .send()
        .await?;

    if StatusCode::NOT_FOUND == response.status() {
        return Ok(None);
    }

    response.json().await.map_err(|e| e.into())
}

pub async fn create_note(
    client: &'static Client,
    config: &'static Config,
    folder_id: i32,
) -> Result<i32, Error> {
    let request_body = serde_json::to_string(&CreateNoteRequest {
        folder_id,
        title: "Newly created note".to_string(),
        text: "".to_string(),
    })
    .unwrap();

    client
        .post(format!("{}/note/note", config.api_url))
        .body(request_body)
        .send()
        .await?
        .error_for_status()?
        .json::<Note>()
        .await
        .map(|note| note.id)
        .map_err(|e| e.into())
}

pub async fn update_note(
    client: &'static Client,
    config: &'static Config,
    note_id: i32,
    text: String,
) -> Result<(), Error> {
    let text = text.as_str().trim().to_string();
    let ending_index = text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0;
    let title = text.as_str()[..ending_index].replace('\n', "");

    let request_body = serde_json::to_string(&UpdateNoteRequest { title, text }).unwrap();

    client
        .put(format!("{}/note/note/{note_id}", config.api_url))
        .body(request_body)
        .send()
        .await?
        .error_for_status()
        .map(|_| ())
        .map_err(|e| e.into())
}
