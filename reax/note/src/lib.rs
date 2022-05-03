pub mod models;

use requests::{CreateFolderRequest, CreateNoteRequest, UpdateNoteRequest};
use reqwest::Client;

use base::{Config, Error};

use models::{Folder, Note};

mod requests;

pub async fn folders(client: &'static Client, config: &'static Config) -> Result<Vec<Folder>, Error> {
    let body = client.get(format!("{}/note/folders", config.api_url))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&body).unwrap())
}

pub async fn create_folder(client: &'static Client, config: &'static Config, name: String) -> Result<(), Error>{
    client.post(format!("{}/note/folder", config.api_url))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&CreateFolderRequest { name }).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(())
}

pub async fn note_summaries(client: &'static Client, config: &'static Config, folder_id: i32) -> Result<Vec<Note>, Error> {
    let body = client.get(format!("{}/note/folder/{}/notes", config.api_url, folder_id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(serde_json::from_str(&body).unwrap())
}

pub async fn note(client: &'static Client, config: &'static Config, note_id: i32) -> Result<Option<Note>, Error> {
    let body = client.get(format!("{}/note/note/{}", config.api_url, note_id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(serde_json::from_str(&body).unwrap())
}

pub async fn create_note(client: &'static Client, config: &'static Config, folder_id: i32) -> Result<i32, Error> {
    let body = client.post(format!("{}/note/note", config.api_url))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&CreateNoteRequest { folder_id, title: String::from("Newly created note"), text: String::from("") }).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(serde_json::from_str::<Note>(&body).unwrap().id)
}

pub async fn update_note(client: &'static Client, config: &'static Config, note_id: i32, text: String) -> Result<(), Error> {
    let text = text.as_str().trim().to_string();
    let title = text.as_str()[..text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0].replace('\n', "");

    client.put(format!("{}/note/note/{note_id}", config.api_url))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UpdateNoteRequest { title, text }).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(())
}
