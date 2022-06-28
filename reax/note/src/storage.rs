use base::Error;
use sqlx::{Sqlite, pool::PoolConnection};

use crate::models::{Folder, Note, NoteState};

pub async fn fetch_folders(conn: &mut PoolConnection<Sqlite>) -> Result<Vec<Folder>, Error> {
    sqlx::query_as("select * from folders order by id")
        .fetch_all(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_folder(conn: &mut PoolConnection<Sqlite>, folder_id: i32) -> Result<Option<Folder>, Error> {
    sqlx::query_as("select * from folders where id = ?")
        .bind(folder_id)
        .fetch_optional(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn create_folder(conn: &mut PoolConnection<Sqlite>, folder: &Folder) -> Result<(), Error> {
    sqlx::query("insert into folders (id, name) values(?, ?)")
        .bind(folder.id)
        .bind(folder.name.as_str())
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn fetch_notes(conn: &mut PoolConnection<Sqlite>, folder_id: i32) -> Result<Vec<Note>, Error> {
    sqlx::query_as("select * from notes where folder_id = ? order by id")
        .bind(folder_id)
        .fetch_all(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_note(conn: &mut PoolConnection<Sqlite>, note_id: i32) -> Result<Option<Note>, Error> {
    sqlx::query_as("select * from notes where id = ?")
        .bind(note_id)
        .fetch_optional(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn create_note(conn: &mut PoolConnection<Sqlite>, note: &Note) -> Result<(), Error> {
    sqlx::query("insert into notes (id, folder_id, title, text, commit_id, state) values(?, ?, ?, ?, ?, ?)")
        .bind(note.id)
        .bind(note.folder_id)
        .bind(note.title.as_ref())
        .bind(note.text.as_str())
        .bind(note.commit_id)
        .bind(&note.state)
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn replace_note(conn: &mut PoolConnection<Sqlite>, note: &Note) -> Result<(), Error> {
    sqlx::query("update notes set title=?, text=?, commit_id=?, state=? where id=?")
        .bind(note.title.as_ref())
        .bind(note.text.as_str())
        .bind(note.commit_id)
        .bind(&note.state)
        .bind(note.id)
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn update_note(conn: &mut PoolConnection<Sqlite>, note_id: i32, title: Option<&str>, text: &str) -> Result<(), Error> {
    sqlx::query("update notes set title=?, text=?, state=? where id=?")
        .bind(title.as_ref())
        .bind(text)
        .bind(NoteState::Modified)
        .bind(note_id)
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}
