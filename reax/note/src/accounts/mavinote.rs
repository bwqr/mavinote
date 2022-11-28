use aes_gcm_siv::{Aes256GcmSiv, Key, KeyInit, Nonce, aead::Aead};
use reqwest::{Client, ClientBuilder, header::{HeaderMap, HeaderValue}, StatusCode};
use base64ct::{Base64, Encoding};

use base::{Error, HttpError, models::Token};

use crate::models::RemoteId;


#[derive(Clone, Debug)]
pub struct MavinoteClient {
    account_id: Option<i32>,
    api_url: String,
    token: String,
    enc_key: Key<Aes256GcmSiv>,
    client: Client,
}

impl MavinoteClient {
    pub fn new(account_id: Option<i32>, api_url: String, token: String, enc_key: Key<Aes256GcmSiv>) -> Self {
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
            enc_key,
            client,
        }
    }

    pub fn with_token(&self, token: String) -> Self {
        Self::new(self.account_id, self.api_url.clone(), token, self.enc_key.clone())
    }

    pub async fn login(api_url: &str, email: &str, password: &str) -> Result<Token, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        let email = email.trim();
        let password = password.trim();

        let request_body = serde_json::to_string(&requests::LoginRequest { email, password }).unwrap();

        client
            .post(format!("{}/auth/login", api_url))
            .body(request_body)
            .send()
            .await?
            .error_for_status()?
            .json::<Token>()
            .await
            .map_err(|e| e.into())

    }

    pub async fn sign_up(api_url: &str, name: &str, email: &str, password: &str) -> Result<Token, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        let name = name.trim();
        let email = email.trim();
        let password = password.trim();

        let request_body = serde_json::to_string(&requests::SignUp { name, email, password }).unwrap();

        let response = client
            .post(format!("{}/auth/sign-up", api_url))
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
            .map(|mut note| {
                if let Some(note) = &mut note {
                    let cipher = Aes256GcmSiv::new(&self.enc_key);
                    let nonce = Nonce::from_slice(b"unique nonce");

                    if let Some(title) = &note.title {
                         match Base64::decode_vec(title.as_str()) {
                            Ok(bytes) => {
                                if let Ok(decrypted) = cipher.decrypt(nonce, bytes.as_ref()) {
                                    note.title = Some(std::str::from_utf8(decrypted.as_ref()).unwrap().to_string());
                                }
                            },
                            Err(e) => log::error!("failed to decode note title as base64 {:?}", e)
                        };
                    }

                    match Base64::decode_vec(note.text.as_str()) {
                        Ok(bytes) => {
                            if let Ok(decrypted) = cipher.decrypt(nonce, bytes.as_ref()) {
                                note.text = std::str::from_utf8(decrypted.as_ref()).unwrap().to_string();
                            }
                        },
                        Err(e) => log::error!("failed to decode note text as base64 {:?}", e)
                    };
                }

                note
            })
            .map_err(|e| self.error(e))
    }

    pub async fn create_note(&self, folder_id: RemoteId, title: Option<&str>, text: &str) -> Result<responses::Note, Error> {
        let cipher = Aes256GcmSiv::new(&self.enc_key);
        let nonce = Nonce::from_slice(b"unique nonce");
        let title = title.map(|t| Base64::encode_string(cipher.encrypt(nonce, t.as_bytes()).unwrap().as_slice()));
        let text = Base64::encode_string(cipher.encrypt(nonce, text.as_bytes()).unwrap().as_slice());

        let request_body = serde_json::to_string(&requests::CreateNoteRequest { folder_id: folder_id.0, title: title.as_ref().map(|t| t.as_str()), text: text.as_str() }).unwrap();

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

    pub async fn update_note(&self, note_id: RemoteId, commit: i32, title: Option<&str>, text: &str) -> Result<responses::Commit, Error> {
        let request = requests::UpdateNoteRequest { commit, title, text };

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
    pub struct LoginRequest<'a> {
        pub email: &'a str,
        pub password: &'a str,
    }

    #[derive(Serialize)]
    pub struct CreateFolderRequest<'a> {
        pub name: &'a str,
    }

    #[derive(Serialize)]
    pub struct CreateNoteRequest<'a> {
        pub folder_id: i32,
        pub title: Option<&'a str>,
        pub text: &'a str,
    }

    #[derive(Serialize)]
    pub struct UpdateNoteRequest<'a> {
        pub commit: i32,
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
