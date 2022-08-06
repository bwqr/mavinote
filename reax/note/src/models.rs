use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Type};

#[derive(Copy, Clone)]
pub struct LocalId(pub i32);
#[derive(Copy, Clone)]
pub struct RemoteId(pub i32);

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub kind: AccountKind,
}

#[derive(Debug, PartialEq, Serialize, Type)]
pub enum AccountKind {
    Mavinote,
    Local
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mavinote {
    pub email: String,
    pub token: String,
}

#[derive(Clone, Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub remote_id: Option<i32>,
    pub title: Option<String>,
    pub text: String,
    pub commit_id: i32,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Type)]
pub enum State {
    Clean,
    Modified,
    Deleted,
}
