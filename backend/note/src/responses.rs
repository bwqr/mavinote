use serde::Serialize;

use crate::models::State;

#[derive(Serialize)]
pub struct Commit {
    pub note_id: i32,
    pub commit: i32,
    pub state: State,
}
