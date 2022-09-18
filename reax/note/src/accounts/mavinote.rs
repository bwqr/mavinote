use reqwest::{Client, ClientBuilder, header::{HeaderMap, HeaderValue}, StatusCode};

use base::{Error, HttpError, models::Token};

use crate::models::RemoteId;


#[derive(Clone)]
pub struct MavinoteClient {
    account_id: Option<i32>,
    api_url: String,
    token: String,
    client: Client,
}

impl MavinoteClient {
    pub fn new(account_id: Option<i32>, api_url: String, token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        MavinoteClient {
            account_id,
            api_url,
            token,
            client,
        }
    }

    pub fn with_token(&self, token: String) -> Self {
        Self::new(self.account_id, self.api_url.clone(), token)
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<Token, Error> {
        let email = email.trim();
        let password = password.trim();

        let request_body = serde_json::to_string(&requests::LoginRequest { email, password }).unwrap();

        self.client
            .post(format!("{}/auth/login", self.api_url))
            .body(request_body)
            .send()
            .await?
            .error_for_status()?
            .json::<Token>()
            .await
            .map_err(|e| e.into())

    }

    pub async fn sign_up(&self, name: &str, email: &str, password: &str) -> Result<Token, Error> {
        let name = name.trim();
        let email = email.trim();
        let password = password.trim();

        let request_body = serde_json::to_string(&requests::SignUp { name, email, password }).unwrap();

        let response = self.client
            .post(format!("{}/auth/sign-up", self.api_url))
            .body(request_body)
            .send()
            .await?;

        if response.status() == StatusCode::CONFLICT {
            return Err(Error::Message("user_with_given_email_already_exists".to_string()))
        }

        response
            .error_for_status()?
            .json::<Token>()
            .await
            .map_err(|e| e.into())
    }

    fn error(&self, e: reqwest::Error) -> Error {
        if let Some(StatusCode::UNAUTHORIZED) = e.status() {
            return Error::Http(HttpError::Unauthorized(self.account_id))
        }

        e.into()
    }
}

impl MavinoteClient {
    pub async fn fetch_folders(&self) -> Result<Vec<responses::Folder>, Error> {
        self.client
            .get(format!("{}/note/folders", self.api_url))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| self.error(e))?
            .json::<Vec<responses::Folder>>()
            .await
            .map_err(|e| self.error(e))
    }

    pub async fn create_folder(&self, folder_name: &str) -> Result<responses::Folder, Error> {
        let request_body = serde_json::to_string(&requests::CreateFolderRequest { name: &folder_name }).unwrap();

        self.client
            .post(format!("{}/note/folder", self.api_url))
            .body(request_body)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| self.error(e))?
            .json::<responses::Folder>()
            .await
            .map_err(|e| self.error(e))
    }

    pub async fn delete_folder(&self, folder_id: RemoteId) -> Result<(), Error> {
        self.client
            .delete(format!("{}/note/folder/{}", self.api_url, folder_id.0))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()
            .map(|_| ())
            .map_err(|e| self.error(e))
    }

    pub async fn fetch_notes(&self, folder_id: i32) -> Result<Vec<responses::Note>, Error> {
        self.client
            .get(format!("{}/note/folder/{}/notes", self.api_url, folder_id))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| self.error(e))?
            .json::<Vec<responses::Note>>()
            .await
            .map_err(|e| self.error(e))
    }

    pub async fn fetch_note(&self, note_id: RemoteId) -> Result<Option<responses::Note>, Error> {
        let response = self.client
            .get(format!("{}/note/note/{}", self.api_url, note_id.0))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        response
            .error_for_status()
            .map_err(|e| self.error(e))?
            .json::<Option<responses::Note>>()
            .await
            .map_err(|e| self.error(e))
    }

    pub async fn create_note(&self, folder_id: RemoteId, title: Option<&str>, text: &str) -> Result<responses::Note, Error> {
        let request_body = serde_json::to_string(&requests::CreateNoteRequest { folder_id: folder_id.0, title, text }).unwrap();

        self.client
            .post(format!("{}/note/note", self.api_url))
            .body(request_body)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| self.error(e))?
            .json::<responses::Note>()
            .await
            .map_err(|e| self.error(e))
    }

    pub async fn update_note(&self, note_id: RemoteId, title: Option<&str>, text: &str) -> Result<responses::Commit, Error> {
        let request = requests::UpdateNoteRequest { title, text };

        self.client
            .put(format!("{}/note/note/{}", self.api_url, note_id.0))
            .header("Authorization", format!("Bearer {}", self.token))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await?
            .error_for_status()
            .map_err(|e| self.error(e))?
            .json::<responses::Commit>()
            .await
            .map_err(|e| self.error(e))
    }

    pub async fn delete_note(&self, note_id: RemoteId) -> Result<(), Error> {
        self.client
            .delete(format!("{}/note/note/{}", self.api_url, note_id.0))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()
            .map(|_| ())
            .map_err(|e| self.error(e))
    }

    pub async fn fetch_commits(&self, folder_id: RemoteId) -> Result<Vec<responses::Commit>, Error> {
        self.client
            .get(format!("{}/note/folder/{}/commits", self.api_url, folder_id.0))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| self.error(e))?
            .json()
            .await
            .map_err(|e| self.error(e))
    }

}

mod requests {
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LoginRequest<'a> {
        pub email: &'a str,
        pub password: &'a str,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateFolderRequest<'a> {
        pub name: &'a str,
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

    #[derive(Serialize)]
    pub struct SignUp<'a> {
        pub name: &'a str,
        pub email: &'a str,
        pub password: &'a str,
    }
}

pub mod responses {
    use serde::{Deserialize, Serialize};

    use crate::models::{RemoteId, State};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Folder {
        pub id: i32,
        pub name: String,
        pub state: State,
    }

    impl Folder {
        pub fn id(&self) -> RemoteId {
            RemoteId(self.id)
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Note {
        pub id: i32,
        pub folder_id: i32,
        pub commit: i32,
        pub title: Option<String>,
        pub text: String,
        pub state: State,
    }

    impl Note {
        pub fn id(&self) -> RemoteId {
            RemoteId(self.id)
        }
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Commit {
        pub note_id: i32,
        pub commit: i32,
        pub state: State,
    }

    impl Commit {
        pub fn note_id(&self) -> RemoteId {
            RemoteId(self.note_id)
        }
    }
}
