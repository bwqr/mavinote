use base::Error;
use sqlx::Connection;
use sqlx::{Sqlite, pool::PoolConnection};

use crate::models::{Folder, Note, State, RemoteId, LocalId};

pub async fn fetch_all_folders(conn: &mut PoolConnection<Sqlite>) -> Result<Vec<Folder>, Error> {
    sqlx::query_as("select * from folders order by id")
        .fetch_all(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_folders(conn: &mut PoolConnection<Sqlite>) -> Result<Vec<Folder>, Error> {
    sqlx::query_as("select * from folders where state != ? order by id")
        .bind(State::Deleted)
        .fetch_all(conn)
        .await
        .map_err(|e| e.into())

}

pub async fn fetch_folder(conn: &mut PoolConnection<Sqlite>, local_id: LocalId) -> Result<Option<Folder>, Error> {
    sqlx::query_as("select * from folders where id = ?")
        .bind(local_id.0)
        .fetch_optional(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_folder_by_remote_id(conn: &mut PoolConnection<Sqlite>, remote_id: RemoteId) -> Result<Option<Folder>, Error> {
    sqlx::query_as("select * from folders where remote_id = ?")
        .bind(remote_id.0)
        .fetch_optional(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn create_folder(conn: &mut PoolConnection<Sqlite>, remote_id: Option<RemoteId>, name: String) -> Result<Folder, Error> {
    conn.transaction(|conn| Box::pin(async move {
        sqlx::query("insert into folders (remote_id, name) values(?, ?)")
            .bind(remote_id.map(|id| id.0))
            .bind(name.as_str())
            .execute(&mut *conn)
            .await
            .map(|_| ())?;

        Ok(sqlx::query_as("select * from folders order by id desc")
            .fetch_optional(conn)
            .await?
            .unwrap())
    }))
     .await
}

pub async fn delete_folder(conn: &mut PoolConnection<Sqlite>, local_id: LocalId) -> Result<(), Error> {
    sqlx::query("delete from folders where id = ?")
        .bind(local_id.0)
        .execute(&mut *conn)
        .await?;

    sqlx::query("delete from notes where folder_id = ?")
        .bind(local_id.0)
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn delete_folder_local(conn: &mut PoolConnection<Sqlite>, local_id: LocalId) -> Result<(), Error> {
    sqlx::query("update folders set state = ? where id = ?")
        .bind(State::Deleted)
        .bind(local_id.0)
        .execute(&mut *conn)
        .await?;

    sqlx::query("delete from notes where folder_id = ?")
        .bind(local_id.0)
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn delete_folder_by_remote_id(conn: &mut PoolConnection<Sqlite>, remote_id: RemoteId) -> Result<(), Error> {
    if let Some(folder) = fetch_folder_by_remote_id(conn, remote_id).await? {
        delete_folder(conn, folder.local_id()).await
    } else {
        Ok(())
    }
}

pub async fn fetch_all_notes(conn: &mut PoolConnection<Sqlite>, folder_id: LocalId) -> Result<Vec<Note>, Error> {
    sqlx::query_as("select * from notes where folder_id = ? order by id")
        .bind(folder_id.0)
        .fetch_all(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_notes(conn: &mut PoolConnection<Sqlite>, folder_id: LocalId) -> Result<Vec<Note>, Error> {
    sqlx::query_as("select * from notes where folder_id = ? and state != ? order by id")
        .bind(folder_id.0)
        .bind(State::Deleted)
        .fetch_all(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_note(conn: &mut PoolConnection<Sqlite>, note_id: LocalId) -> Result<Option<Note>, Error> {
    sqlx::query_as("select * from notes where id = ?")
        .bind(note_id.0)
        .fetch_optional(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_note_by_remote_id(conn: &mut PoolConnection<Sqlite>, note_id: RemoteId) -> Result<Option<Note>, Error> {
    sqlx::query_as("select * from notes where remote_id = ?")
        .bind(note_id.0)
        .fetch_optional(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn create_note(conn: &mut PoolConnection<Sqlite>, folder_id: LocalId, remote_id: Option<RemoteId>, title: Option<String>, text: String, commit_id: i32) -> Result<Note, Error> {
    conn.transaction(|conn| Box::pin(async move {
        sqlx::query("insert into notes (folder_id, remote_id, title, text, commit_id, state) values(?, ?, ?, ?, ?, ?)")
            .bind(folder_id.0)
            .bind(remote_id.map(|id| id.0))
            .bind(title.as_ref())
            .bind(text.as_str())
            .bind(commit_id)
            .bind(State::Clean)
            .execute(&mut *conn)
            .await?;

        Ok(sqlx::query_as("select * from notes order by id desc")
            .fetch_optional(conn)
            .await?
            .unwrap())
    }))
     .await
}

pub async fn update_note(conn: &mut PoolConnection<Sqlite>, note_id: LocalId, title: Option<&str>, text: &str, commit_id: i32, state: State) -> Result<(), Error> {
    sqlx::query("update notes set title=?, text=?, commit_id=?, state=? where id=?")
        .bind(title)
        .bind(text)
        .bind(commit_id)
        .bind(state)
        .bind(note_id.0)
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn update_commit(conn: &mut PoolConnection<Sqlite>, note_id: LocalId, commit_id: i32) -> Result<(), Error> {
    sqlx::query("update notes set commit_id=?, state=? where id=?")
        .bind(commit_id)
        .bind(State::Clean)
        .bind(note_id.0)
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn delete_note(conn: &mut PoolConnection<Sqlite>, local_id: LocalId) -> Result<(), Error> {
    sqlx::query("delete from notes where id = ?")
        .bind(local_id.0)
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}

pub async fn delete_note_local(conn: &mut PoolConnection<Sqlite>, local_id: LocalId) -> Result<(), Error> {
    sqlx::query("update notes set state = ? where id = ?")
        .bind(State::Deleted)
        .bind(local_id.0)
        .execute(&mut *conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}
