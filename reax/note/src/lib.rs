pub mod models;

use std::sync::Arc;

use requests::{CreateFolderRequest, CreateNoteRequest, UpdateNoteRequest};
use reqwest::{Client, StatusCode};
use once_cell::sync::OnceCell;
use tokio::sync::watch::channel;

use base::{Config, Error, Store, State, observable_map::{ObservableMap, Receiver}};

use models::{Folder, Note};

mod requests;

type Sender<T> = tokio::sync::watch::Sender<State<T, Error>>;

static FOLDERS: OnceCell<Sender<Vec<Folder>>> = OnceCell::new();
static NOTES_MAP: OnceCell<Arc<ObservableMap<State<Vec<Note>, Error>>>> = OnceCell::new();

pub fn init() {
    FOLDERS.set(channel(State::default()).0).unwrap();
    NOTES_MAP.set(Arc::new(ObservableMap::new())).unwrap();
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

pub async fn folders() -> tokio::sync::watch::Receiver<State<Vec<Folder>, Error>> {
    let sender = FOLDERS.get().unwrap();
    let load = match *sender.borrow() {
        State::Initial | State::Err(_) => true,
        _ => false,
    };

    if load {
        sender.send_replace(State::Loading);
        sender.send_replace(fetch_folders().await.into());
    }

    sender.subscribe()
}

pub async fn create_folder(name: String) -> Result<(), Error> {
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
        .error_for_status()
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn fetch_notes(folder_id: i32) -> Result<Vec<Note>, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    client
        .get(format!("{}/note/folder/{}/notes", config.api_url, folder_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(|e| e.into())
}

pub async fn notes(folder_id: i32) -> Receiver<State<Vec<Note>, Error>> {
    let notes_map = NOTES_MAP.get().unwrap();

    if !notes_map.contains_key(folder_id) {
        notes_map.insert(folder_id, fetch_notes(folder_id).await.into());
    }

    notes_map.subscribe(folder_id).unwrap()
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

pub async fn create_note(folder_id: i32) -> Result<i32, Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = store.get("token").await?.unwrap_or("".to_string());

    let request_body = serde_json::to_string(&CreateNoteRequest {
        folder_id,
        title: "Newly created note".to_string(),
        text: "".to_string(),
    })
    .unwrap();

    let note = client
        .post(format!("{}/note/note", config.api_url))
        .header("Authorization", format!("Bearer {}", token))
        .body(request_body)
        .send()
        .await?
        .error_for_status()?
        .json::<Note>()
        .await?;
    let note_id = note.id;

    NOTES_MAP.get().unwrap().update_modify(folder_id, move |state| {
        if let State::Ok(notes) = state {
            notes.push(note);
        }
    });

    Ok(note_id)
}

pub async fn update_note(note_id: i32, folder_id: i32, text: String) -> Result<(), Error> {
    let store = runtime::get::<Arc<dyn Store>>().unwrap();
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    let token = store.get("token").await?.unwrap_or("".to_string());

    let text = text.as_str().trim().to_string();
    let ending_index = text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0;
    let title = text.as_str()[..ending_index].replace('\n', "");

    let request = UpdateNoteRequest { title, text };

    client
        .put(format!("{}/note/note/{note_id}", config.api_url))
        .header("Authorization", format!("Bearer {}", token))
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await?
        .error_for_status()?;

    NOTES_MAP.get().unwrap().update_modify(folder_id, move |state| {
        if let State::Ok(notes) = state {
            if let Some(mut note) = notes.iter_mut().find(|n| n.id == note_id) {
                note.text = request.text;
                note.title=  request.title;
            }
        }
    });

    Ok(())
}
