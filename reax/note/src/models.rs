use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[cfg(feature = "storage")]
use sqlx::{FromRow, Type};

#[derive(Copy, Clone)]
pub struct LocalId(pub i32);
#[derive(Copy, Clone)]
pub struct RemoteId(pub i32);

#[cfg_attr(feature = "storage", derive(Type))]
pub enum StoreKey {
    Version,
    IdentityPrivKey,
    IdentityPubKey,
    Password,
    WelcomeShown,
    NonceId,
}

#[cfg_attr(feature = "storage", derive(FromRow))]
pub struct StoreValue {
    pub key: StoreKey,
    pub value: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "storage", derive(FromRow))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub kind: AccountKind,
}

#[derive(Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "storage", derive(Type))]
pub enum AccountKind {
    Mavinote,
    Local
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "storage", derive(FromRow))]
pub struct Device {
    pub id: i32,
    pub account_id: i32,
    pub pubkey: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mavinote {
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "storage", derive(FromRow))]
pub struct Folder {
    pub id: i32,
    pub account_id: i32,
    pub remote_id: Option<i32>,
    pub name: String,
    pub state: State,
}

impl Folder {
    pub fn local_id(&self) -> LocalId {
        LocalId(self.id)
    }

    pub fn remote_id(&self) -> Option<RemoteId> {
        self.remote_id.map(|id| RemoteId(id))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "storage", derive(FromRow))]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub remote_id: Option<i32>,
    pub commit: i32,
    pub name: String,
    pub text: String,
    pub state: State,
}

impl Note {
    pub fn local_id(&self) -> LocalId {
        LocalId(self.id)
    }

    pub fn remote_id(&self) -> Option<RemoteId> {
        self.remote_id.map(|id| RemoteId(id))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "storage", derive(Type))]
pub enum State {
    Clean,
    Modified,
    Deleted,
}
