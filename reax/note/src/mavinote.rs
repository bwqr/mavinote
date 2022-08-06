use base::{Error, HttpError};
use reqwest::{Client as ReqwestClient, ClientBuilder, header::{HeaderMap, HeaderValue}, StatusCode};

use crate::{responses::{Folder, Commit, Note}, models::RemoteId, requests::{CreateFolderRequest, CreateNoteRequest, UpdateNoteRequest}};

pub struct Client {
    account_id: i32,
    client: ReqwestClient,
    endpoint: String,
    token: String,
}

impl Client {
    pub fn new(account_id: i32, endpoint: String, token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            account_id,
            client,
            endpoint,
            token,
        }
    }

    pub async fn fetch_folders(&self) -> Result<Vec<Folder>, Error> {
        self.client
            .get(format!("{}/note/folders", self.endpoint))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| self.error(e))?
            .json::<Vec<Folder>>()
            .await
            .map_err(|e| self.error(e))
    }

    pub async fn create_folder(&self, name: &str) -> Result<Folder, Error> {
        self.client
            .post(format!("{}/note/folder", self.endpoint))
            .header("Authorization", format!("Bearer {}", self.token))
            .body(serde_json::to_string(&CreateFolderRequest { name }).unwrap())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(|e| self.error(e))
    }

    pub async fn delete_folder(&self, folder_id: RemoteId) -> Result<(), Error> {
        self.client
            .delete(format!("{}/note/folder/{}", self.endpoint, folder_id.0))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()
            .map(|_| ())
            .map_err(|e| self.error(e))
    }

    pub async fn fetch_commits(&self, folder_id: RemoteId) -> Result<Vec<Commit>, Error> {
        self.client
            .get(format!("{}/note/folder/{}/commits", self.endpoint, folder_id.0))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| self.error(e))?
            .json()
            .await
            .map_err(|e| self.error(e))
    }

pub async fn fetch_note(&self, note_id: RemoteId) -> Result<Note, Error> {
    self.client
        .get(format!("{}/note/note/{}", self.endpoint, note_id.0))
        .header("Authorization", format!("Bearer {}", self.token))
        .send()
        .await?
        .error_for_status()
        .map_err(|e| self.error(e))?
        .json()
        .await
        .map_err(|e| self.error(e))
}

pub async fn create_note(&self, folder_id: RemoteId, title: Option<&str>, text: &str) -> Result<Note, Error> {
    let request_body = serde_json::to_string(&CreateNoteRequest {
        folder_id: folder_id.0,
        title,
        text,
    })
    .unwrap();

    self.client
        .post(format!("{}/note/note", self.endpoint))
        .header("Authorization", format!("Bearer {}", self.token))
        .body(request_body)
        .send()
        .await?
        .error_for_status()
        .map_err(|e| self.error(e))?
        .json::<Note>()
        .await
        .map_err(|e| self.error(e))
}

pub async fn update_note(&self, note_id: RemoteId, title: Option<&str>, text: &str) -> Result<Commit, Error> {
    let request = UpdateNoteRequest { title, text };

    self.client
        .put(format!("{}/note/note/{}", self.endpoint, note_id.0))
        .header("Authorization", format!("Bearer {}", self.token))
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await?
        .error_for_status()
        .map_err(|e| self.error(e))?
        .json()
        .await
        .map_err(|e| self.error(e))
}

pub async fn delete_note(&self, note_id: RemoteId) -> Result<(), Error> {
    self.client
        .delete(format!("{}/note/note/{}", self.endpoint, note_id.0))
        .header("Authorization", format!("Bearer {}", self.token))
        .send()
        .await?
        .error_for_status()
        .map(|_| ())
        .map_err(|e| self.error(e))
}

    fn error(&self, e: reqwest::Error) -> Error {
        if let Some(StatusCode::UNAUTHORIZED) = e.status() {
            return Error::Http(HttpError::Unauthorized(Some(self.account_id)))
        }

        e.into()
    }
}
