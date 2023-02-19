use serde::Serialize;

pub mod accounts;
pub mod models;

#[cfg(feature = "storage")]
pub mod storage;


#[derive(Clone, Debug, Serialize)]
pub enum Error {
    Mavinote(accounts::mavinote::Error),
    Storage(StorageError),
    Database(String),
}

#[derive(Clone, Debug, Serialize)]
pub enum StorageError {
    InvalidState(String),
    NotMavinoteAccount,
    AccountNotFound,
    AccountEmailUsed,
    FolderNotFound,
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
