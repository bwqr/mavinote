pub mod models;

use std::sync::Arc;

use requests::{CreateFolderRequest, CreateNoteRequest, UpdateNoteRequest};
use reqwest::{Client, StatusCode};

use base::{Config, Error, Store};

use models::{Folder, Note};

mod requests;

pub async fn folders(
    store: Arc<dyn Store>,
    client: Arc<Client>,
    config: Arc<Config>,
) -> Result<Vec<Folder>, Error> {
    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .get(format!("{}/note/folders", config.api_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn create_folder(
    store: Arc<dyn Store>,
    client: Arc<Client>,
    config: Arc<Config>,
    name: String,
) -> Result<(), Error> {
    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .post(format!("{}/note/folder", config.api_url))
        .header("Authorization", format!("Bearer {}", token))
        .body(serde_json::to_string(&CreateFolderRequest { name }).unwrap())
        .send()
        .await?
        .error_for_status()
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn note_summaries(
    store: Arc<dyn Store>,
    client: Arc<Client>,
    config: Arc<Config>,
    folder_id: i32,
) -> Result<Vec<Note>, Error> {
    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .get(format!(
            "{}/note/folder/{}/notes",
            config.api_url, folder_id
        ))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn note(
    store: Arc<dyn Store>,
    client: Arc<Client>,
    config: Arc<Config>,
    note_id: i32,
) -> Result<Option<Note>, Error> {
    let token = store.get("token").await?.unwrap_or("".to_string());

    let response = client
        .get(format!("{}/note/note/{}", config.api_url, note_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if StatusCode::NOT_FOUND == response.status() {
        return Ok(None);
    }

    response
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn create_note(
    store: Arc<dyn Store>,
    client: Arc<Client>,
    config: Arc<Config>,
    folder_id: i32,
) -> Result<i32, Error> {
    let token = store.get("token").await?.unwrap_or("".to_string());

    let request_body = serde_json::to_string(&CreateNoteRequest {
        folder_id,
        title: "Newly created note".to_string(),
        text: "".to_string(),
    })
    .unwrap();

    client
        .post(format!("{}/note/note", config.api_url))
        .header("Authorization", format!("Bearer {}", token))
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
    store: Arc<dyn Store>,
    client: Arc<Client>,
    config: Arc<Config>,
    note_id: i32,
    text: String,
) -> Result<(), Error> {
    let token = store.get("token").await?.unwrap_or("".to_string());

    let text = text.as_str().trim().to_string();
    let ending_index = text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0;
    let title = text.as_str()[..ending_index].replace('\n', "");

    let request_body = serde_json::to_string(&UpdateNoteRequest { title, text }).unwrap();

    client
        .put(format!("{}/note/note/{note_id}", config.api_url))
        .header("Authorization", format!("Bearer {}", token))
        .body(request_body)
        .send()
        .await?
        .error_for_status()
        .map(|_| ())
        .map_err(|e| e.into())
}
