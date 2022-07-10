use std::sync::Arc;

use base::{Error, Store, Config};
use reqwest::Client;
use serde::Serialize;

use crate::{responses::{Folder, Note, Commit}, models::RemoteId};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateFolderRequest<'a> {
    pub name: &'a str
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateNoteRequest<'a> {
    pub folder_id: i32,
    pub title: Option<&'a str>,
    pub text: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateNoteRequest<'a> {
    pub title: Option<&'a str>,
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

pub async fn create_folder(name: &str) -> Result<Folder, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .post(format!("{}/note/folder", config.api_url))
        .header("Authorization", format!("Bearer {}", token))
        .body(serde_json::to_string(&CreateFolderRequest { name }).unwrap())
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn delete_folder(folder_id: RemoteId) -> Result<(), Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .delete(format!("{}/note/folder/{}", config.api_url, folder_id.0))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn fetch_commits(folder_id: RemoteId) -> Result<Vec<Commit>, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .get(format!("{}/note/folder/{}/commits", config.api_url, folder_id.0))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_note(note_id: RemoteId) -> Result<Note, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .get(format!("{}/note/note/{}", config.api_url, note_id.0))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn create_note(folder_id: RemoteId, title: Option<&str>, text: &str) -> Result<Note, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    let request_body = serde_json::to_string(&CreateNoteRequest {
        folder_id: folder_id.0,
        title,
        text,
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

pub async fn update_note(note_id: RemoteId, title: Option<&str>, text: &str) -> Result<Commit, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    let request = UpdateNoteRequest { title, text };

    client
        .put(format!("{}/note/note/{}", config.api_url, note_id.0))
        .header("Authorization", format!("Bearer {}", token))
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn delete_note(note_id: RemoteId) -> Result<(), Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .delete(format!("{}/note/note/{}", config.api_url, note_id.0))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()
        .map(|_| ())
        .map_err(|e| e.into())
}
