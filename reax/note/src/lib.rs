use crypto::CryptoError;
use serde::Serialize;

pub mod accounts;
pub mod models;

#[cfg(feature = "storage")]
pub mod storage;

mod crypto;


#[derive(Clone, Debug, Serialize)]
pub enum Error {
    Mavinote(accounts::mavinote::Error),
    Storage(StorageError),
    Database(String),
    Crypto(CryptoError),
}

#[derive(Clone, Debug, Serialize)]
pub enum StorageError {
    InvalidState(String),
    NotMavinoteAccount,
    AccountNotFound,
    AccountEmailUsed,
    FolderNotFound,
    NoteNotFound,
}

#[cfg(feature = "storage")]
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Database(e.to_string())
    }
}

impl From<accounts::mavinote::Error> for Error {
    fn from(e: accounts::mavinote::Error) -> Self {
        Error::Mavinote(e)
    }
}

impl From<StorageError> for Error {
    fn from(e: StorageError) -> Self {
        Error::Storage(e)
    }
}

impl From<CryptoError> for Error {
    fn from(e: CryptoError) -> Self {
        Error::Crypto(e)
    }
}
