use std::{future::Future, time::Duration};

use reqwest::{Client, ClientBuilder, header::{HeaderMap, HeaderValue}, StatusCode};
use serde::{Deserialize, Serialize};
use futures_util::{StreamExt, SinkExt};
use tokio::time::Instant;
use tokio_tungstenite::connect_async;

use crate::models::RemoteId;

pub use requests::{CreateFolderRequest, CreateNoteRequest, RespondRequests, RespondFolderRequest, RespondNoteRequest, CreateRequests};
pub use responses::Device;

#[derive(Debug, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum DeviceMessage {
    AcceptPendingDevice,
    RefreshRequests,
    RefreshRemote,
    RefreshFolder(i32),
    RefreshNote { folder_id: i32, note_id: i32 },
    Text(String),
    Timeout,
}

#[derive(Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Deserialize)]
struct HttpError {
    pub error: String,
}

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    Unauthorized(Option<i32>),
    Message(String),
    NoConnection,
    UnexpectedResponse,
    DeviceDeleted(i32),
    Unknown(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if let Some(StatusCode::UNAUTHORIZED) = e.status() {
            return Error::Unauthorized(None)
        }

        #[cfg(not(target_arch = "wasm32"))]
        if e.is_connect() {
            return Error::NoConnection
        }

        if e.is_decode() {
            return Error::UnexpectedResponse
        }

        Error::Unknown(format!("{e:?}"))
    }
}

pub struct AuthClient {
    api_url: String,
    client: Client,
}

impl AuthClient {
    pub fn new(api_url: String) -> AuthClient {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = ClientBuilder::new()
            .default_headers(headers)
            .connect_timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();

        AuthClient {
            api_url,
            client,
        }
    }

    async fn error_for_status(response: reqwest::Response) -> Result<reqwest::Response, Error> {
        let status = response.status();

        if status.is_success() {
            return Ok(response);
        }

        if status == StatusCode::UNAUTHORIZED {
            return Err(Error::Unauthorized(None));
        }

        let error = response.json::<HttpError>().await?.error;

        Err(Error::Message(error))
    }

    pub async fn login(&self, email: &str, pubkey: &str, password: &str) -> Result<Token, Error> {
        let request = requests::Login { email, pubkey, password };

        self.client
            .post(format!("{}/auth/login", self.api_url))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
            .map(Self::error_for_status)?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn wait_verification(ws_url: &str, token: &str) -> Result<(), Error> {
        let ws_failed = || Error::Message("ws_failed".to_string());

        let (mut sock, _) = connect_async(format!("{}/auth/wait-verification?token={}", ws_url, token)).await
            .map_err(|e| {
                log::debug!("failed to establish ws connection, {:?}", e);

                ws_failed()
            })?;

        let mut instant = Instant::now();

        const PING_INTERVAL: u64 = 30;
        loop {
            if Instant::now().duration_since(instant).as_secs() > PING_INTERVAL {
                instant = Instant::now();
                if let Err(e) = sock.send("ping".into()).await {
                    log::error!("failed to ping over socket, {e:?}");
                }
            }

            let Ok(frame) = tokio::time::timeout_at(instant + Duration::from_secs(PING_INTERVAL), sock.next()).await else {
                continue;
            };

            match frame {
                Some(Ok(msg)) => {
                    match msg.into_text() {
                        Ok(msg) => {
                            match serde_json::from_str::<DeviceMessage>(&msg) {
                                Ok(DeviceMessage::AcceptPendingDevice) => return Ok(()),
                                Ok(DeviceMessage::Timeout) => return Err(Error::Message("ws_timeout".to_string())),
                                Ok(msg) => log::debug!("unexpected device message is received {msg:?}"),
                                Err(e) => log::debug!("failed to deserialize device message {e:?}"),
                            };
                        },
                        Err(e) => log::debug!("non text message is received, {:?}", e),
                    }
                }
                Some(Err(e)) => log::debug!("failed to receive message, {:?}", e),
                None => log::debug!("no message is received"),
            }

            return Err(ws_failed());
        }
    }

    pub async fn request_verification(&self, email: &str, pubkey: &str, password: &str) -> Result<Token, Error> {
        let request = requests::RequestVerification { email, pubkey, password };

        self.client
            .post(format!("{}/auth/request-verification", self.api_url))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
            .map(Self::error_for_status)?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn send_verification_code(&self, email: &str) -> Result<(), Error> {
        let request = requests::SendCode { email };

        self.client
            .post(format!("{}/auth/send-code", self.api_url))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
            .map(Self::error_for_status)?
            .await
            .map(|_| ())
    }

    pub async fn sign_up(&self, email: &str, code: &str, pubkey: &str, password: &str) -> Result<Token, Error> {
        let request = requests::SignUp { email, code, pubkey, password };

        self.client
            .post(format!("{}/auth/sign-up", self.api_url))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
            .map(Self::error_for_status)?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }
}

#[derive(Clone, Debug)]
pub struct MavinoteClient {
    account_id: i32,
    api_url: String,
    client: Client,
}

impl MavinoteClient {
    pub fn new(account_id: i32, api_url: String, token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        MavinoteClient {
            account_id,
            api_url,
            client,
        }
    }

    async fn error_for_status(&self, response: reqwest::Response) -> Result<reqwest::Response, Error> {
        let status = response.status();

        if status.is_success() {
            return Ok(response);
        }

        let error = response.json::<HttpError>().await?.error;

        if status == StatusCode::UNAUTHORIZED {
            if error == "device_deleted" {
                return Err(Error::DeviceDeleted(self.account_id));
            }

            return Err(Error::Unauthorized(Some(self.account_id)));
        }

        Err(Error::Message(error))
    }

    pub async fn login_on_unauthorized<T, R, L>(
        self,
        future_callback: &(dyn Sync + Fn(Self) -> R),
        login: &(dyn Sync + Fn(i32) -> L)
    ) -> Result<T, Error>
    where
        R: Future<Output = Result<T, Error>>,
        L: Future<Output = Result<Token, crate::Error>>,
    {
        let (account_id, api_url) = (self.account_id, self.api_url.clone());

        let token = match future_callback(self).await {
            Err(Error::Unauthorized(Some(account_id))) => {
                match login(account_id).await {
                    Ok(token) => token.token,
                    Err(e) => {
                        log::debug!("Failed to login after receiving unauthorized response, {e:?}");

                        return Err(Error::Unauthorized(Some(account_id)))
                    }
                }
            }
            result => return result,
        };

        future_callback(MavinoteClient::new(account_id, api_url, token)).await
    }
}

impl MavinoteClient {
    pub async fn send_account_close_code(&self) -> Result<(), Error> {
        self.client
            .post(format!("{}/user/send-close-code", self.api_url))
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await
            .map(|_| ())
    }

    pub async fn close_account(&self, code: &str) -> Result<(), Error> {
        self.client
            .put(format!("{}/user/close", self.api_url))
            .body(serde_json::to_string(&requests::CloseAccount { code }).unwrap())
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await
            .map(|_| ())
    }

    pub async fn fetch_devices(&self) -> Result<Vec<responses::Device>, Error> {
        self.client
            .get(format!("{}/user/devices", self.api_url))
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn add_device(&self, pubkey: &str) -> Result<responses::Device, Error> {
        let request = requests::AddDevice { pubkey };

        self.client
            .post(format!("{}/user/device", self.api_url))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_device(&self, device_id: Option<i32>) -> Result<(), Error> {
        let mut url = format!("{}/user/device", self.api_url);

        if let Some(device_id) = device_id {
            url += format!("?id={}", device_id).as_str();
        }

        self.client
            .delete(url)
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await
            .map(|_| ())
    }

    pub async fn fetch_folders(&self) -> Result<Vec<responses::Folder>, Error> {
        self.client
            .get(format!("{}/note/folders", self.api_url))
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn fetch_folder(&self, folder_id: RemoteId) -> Result<Option<responses::Folder>, Error> {
        let response = self.client
            .get(format!("{}/note/folder/{}", self.api_url, folder_id.0))
            .send()
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        self.error_for_status(response)
            .await?
            .json()
            .await
            .map_err(|e| e.into())

    }

    pub async fn create_folder(&self, request: &[requests::CreateFolderRequest]) -> Result<responses::CreatedFolder, Error> {
        self.client
            .post(format!("{}/note/folder", self.api_url))
            .body(serde_json::to_string(request).unwrap())
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_folder(&self, folder_id: RemoteId) -> Result<(), Error> {
        self.client
            .delete(format!("{}/note/folder/{}", self.api_url, folder_id.0))
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await
            .map(|_| ())
    }

    pub async fn fetch_note(&self, note_id: RemoteId) -> Result<Option<responses::Note>, Error> {
        let response = self.client
            .get(format!("{}/note/note/{}", self.api_url, note_id.0))
            .send()
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        self.error_for_status(response)
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn create_note(&self, folder_id: RemoteId, device_notes: &[requests::CreateNoteRequest]) -> Result<responses::CreatedNote, Error> {
        self.client
            .post(format!("{}/note/note?folder_id={}", self.api_url, folder_id.0))
            .body(serde_json::to_string(&device_notes).unwrap())
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn update_note(&self, note_id: RemoteId, commit: i32, device_notes: &[requests::CreateNoteRequest]) -> Result<responses::Commit, Error> {
        let request = requests::UpdateNoteRequest { commit,  device_notes };

        self.client
            .put(format!("{}/note/note/{}", self.api_url, note_id.0))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_note(&self, note_id: RemoteId) -> Result<(), Error> {
        self.client
            .delete(format!("{}/note/note/{}", self.api_url, note_id.0))
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await
            .map(|_| ())
    }

    pub async fn fetch_commits(&self, folder_id: RemoteId) -> Result<Vec<responses::Commit>, Error> {
        self.client
            .get(format!("{}/note/folder/{}/commits", self.api_url, folder_id.0))
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn fetch_requests(&self) -> Result<responses::Requests, Error> {
        self.client
            .get(format!("{}/note/requests", self.api_url))
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn respond_requests(&self, request: &RespondRequests) -> Result<(), Error> {
        self.client
            .post(format!("{}/note/respond-requests", self.api_url))
            .body(serde_json::to_string(request).unwrap())
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await
            .map(|_| ())
    }

    pub async fn create_requests(&self, request: &CreateRequests) -> Result<(), Error> {
        self.client
            .post(format!("{}/note/requests", self.api_url))
            .body(serde_json::to_string(request).unwrap())
            .send()
            .await
            .map(|r| async { self.error_for_status(r).await })?
            .await
            .map(|_| ())
    }
}

mod requests {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Login<'a> {
        pub email: &'a str,
        pub pubkey: &'a str,
        pub password: &'a str,
    }

    #[derive(Serialize)]
    pub struct CloseAccount<'a> {
        pub code: &'a str
    }

    #[derive(Serialize)]
    pub struct CreateFolderRequest {
        pub name: String,
        pub device_id: i32,
    }

    #[derive(Serialize)]
    pub struct CreateNoteRequest {
        pub name: String,
        pub text: String,
        pub device_id: i32,
    }

    #[derive(Serialize)]
    pub struct UpdateNoteRequest<'a> {
        pub commit: i32,
        pub device_notes: &'a [CreateNoteRequest],
    }

    #[derive(Serialize)]
    pub struct SignUp<'a> {
        pub email: &'a str,
        pub code: &'a str,
        pub pubkey: &'a str,
        pub password: &'a str,
    }

    #[derive(Serialize)]
    pub struct RequestVerification<'a> {
        pub email: &'a str,
        pub pubkey: &'a str,
        pub password: &'a str,
    }

    #[derive(Serialize)]
    pub struct SendCode<'a> {
        pub email: &'a str,
    }

    #[derive(Serialize)]
    pub struct AddDevice<'a> {
        pub pubkey: &'a str,
    }

    #[derive(Serialize)]
    pub struct RespondRequests {
        pub device_id: i32,
        pub folders: Vec<RespondFolderRequest>,
        pub notes: Vec<RespondNoteRequest>,
    }

    #[derive(Serialize)]
    pub struct RespondFolderRequest {
        pub folder_id: i32,
        pub name: String,
    }

    #[derive(Serialize)]
    pub struct RespondNoteRequest {
        pub note_id: i32,
        pub name: String,
        pub text: String,
    }

    #[derive(Default, Serialize)]
    pub struct CreateRequests {
        pub folder_ids: Vec<i32>,
        pub note_ids: Vec<i32>,
    }
}

pub mod responses {
    use chrono::NaiveDateTime;
    use serde::Deserialize;

    use crate::models::{RemoteId, State};

    #[derive(Deserialize)]
    pub struct CreatedFolder {
        pub id: i32,
    }

    impl CreatedFolder {
        pub fn id(&self) -> RemoteId {
            RemoteId(self.id)
        }
    }

    #[derive(Deserialize)]
    pub struct Device {
        pub id: i32,
        pub pubkey: String,
        pub created_at: NaiveDateTime,
    }

    #[derive(Debug, Deserialize)]
    pub struct Folder {
        pub id: i32,
        pub state: State,
        pub device_folder: Option<DeviceFolder>,
    }
    impl Folder {
        pub fn id(&self) -> RemoteId {
            RemoteId(self.id)
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct DeviceFolder {
        pub sender_device_id: i32,
        pub name: String,
    }

    #[derive(Deserialize)]
    pub struct CreatedNote {
        pub id: i32,
        pub commit: i32,
    }

    impl CreatedNote {
        pub fn id(&self) -> RemoteId {
            RemoteId(self.id)
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Note {
        pub id: i32,
        pub commit: i32,
        pub state: State,
        pub device_note: Option<DeviceNote>,
    }

    impl Note {
        pub fn id(&self) -> RemoteId {
            RemoteId(self.id)
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct DeviceNote {
        pub sender_device_id: i32,
        pub name: String,
        pub text: String,
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

    #[derive(Deserialize)]
    pub struct Requests {
        pub folder_requests: Vec<FolderRequest>,
        pub note_requests: Vec<NoteRequest>,
    }

    #[derive(Deserialize)]
    pub struct FolderRequest {
        pub folder_id: i32,
        pub device_id: i32,
    }

    #[derive(Deserialize)]
    pub struct NoteRequest {
        pub note_id: i32,
        pub device_id: i32,
    }
}
