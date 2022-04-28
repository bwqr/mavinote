pub mod models;

use sqlx::{Sqlite, pool::PoolConnection};

use models::{Folder, Note, NoteSummary};

pub async fn folders(mut conn: PoolConnection<Sqlite>) -> Vec<Folder> {
    sqlx::query_as("select * from folders order by id")
        .fetch_all(&mut conn)
        .await
        .unwrap()
}

pub async fn note_summaries(mut conn: PoolConnection<Sqlite>, folder_id: i32) -> Vec<Note> {
    sqlx::query_as("select * from notes where folder_id = ? order by id")
        .bind(folder_id)
        .fetch_all(&mut conn)
        .await
        .unwrap()
}

pub async fn note(note_id: i32) -> Option<Note> {
    Some(Note {
        id: note_id,
        folder_id: 1,
        title: String::from("Note Title"),
        text: String::from("Note Text"),
    })
}

pub async fn update_note(note_id: i32, text: String) {
}
