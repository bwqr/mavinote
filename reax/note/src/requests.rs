use std::sync::Arc;

use base::{Error, Store, Config};
use reqwest::{Client, StatusCode};
use serde::Serialize;

use crate::{models::{Folder, Note}, responses::Commit};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateFolderRequest<'a> {
    pub name: &'a str
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateNoteRequest<'a> {
    pub folder_id: i32,
    pub title: &'a str,
    pub text: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateNoteRequest<'a> {
    pub title: &'a str,
    pub text: &'a str,
}

pub async fn fetch_folders() -> Result<Vec<Folder>, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

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

pub async fn create_folder(name: String) -> Result<Folder, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .post(format!("{}/note/folder", config.api_url))
        .header("Authorization", format!("Bearer {}", token))
        .body(serde_json::to_string(&CreateFolderRequest { name: &name }).unwrap())
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_commits(folder_id: i32) -> Result<Vec<Commit>, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .get(format!("{}/note/folder/{}/commits", config.api_url, folder_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn note(note_id: i32) -> Result<Option<Note>, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

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

pub async fn create_note(folder_id: i32) -> Result<Note, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    let request_body = serde_json::to_string(&CreateNoteRequest {
        folder_id,
        title: "Newly created note",
        text: "",
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
        .map_err(|e| e.into())
}

pub async fn update_note(note_id: i32, title: &str, text: &str) -> Result<(), Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    let request = UpdateNoteRequest { title, text };

    client
        .put(format!("{}/note/note/{note_id}", config.api_url))
        .header("Authorization", format!("Bearer {}", token))
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
