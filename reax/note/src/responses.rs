use serde::Deserialize;

use crate::models::State;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub commit_id: i32,
    pub note_id: i32,
    pub state: State,
}
