use sqlx::Connection;
use sqlx::{Sqlite, pool::PoolConnection, Error};

use crate::models::{Folder, Note, State, RemoteId, LocalId, Account, AccountKind};

pub async fn fetch_accounts(conn: &mut PoolConnection<Sqlite>) -> Result<Vec<Account>, Error> {
    sqlx::query_as("select * from accounts order by id")
        .fetch_all(conn)
        .await
}

pub async fn fetch_account(conn: &mut PoolConnection<Sqlite>, account_id: i32) -> Result<Option<Account>, Error> {
    sqlx::query_as("select * from accounts where id = ?")
        .bind(account_id)
        .fetch_optional(conn)
        .await
}

pub async fn fetch_account_folders(conn: &mut PoolConnection<Sqlite>, account_id: i32) -> Result<Vec<Folder>, Error> {
    sqlx::query_as("select * from folders where account_id = ? order by id")
        .bind(account_id)
        .fetch_all(conn)
        .await
}

pub async fn create_account(conn: &mut PoolConnection<Sqlite>, name: String, kind: AccountKind) -> Result<Account, Error> {
    conn.transaction(|conn| Box::pin(async move {
        sqlx::query("insert into accounts (name, kind) values(?, ?)")
            .bind(name)
            .bind(kind)
            .execute(&mut *conn)
            .await
            .map(|_| ())?;

        sqlx::query_as("select * from accounts order by id desc")
            .fetch_optional(conn)
            .await
            .map(|opt| opt.unwrap())
    }))
     .await
}

pub async fn delete_account(conn: &mut PoolConnection<Sqlite>, account_id: i32) -> Result<(), Error> {
    sqlx::query("delete from accounts where id = ?")
        .bind(account_id)
        .execute(&mut *conn)
        .await
        .map(|_| ())
}

pub async fn fetch_folders(conn: &mut PoolConnection<Sqlite>) -> Result<Vec<Folder>, Error> {
    sqlx::query_as("select * from folders where state != ? order by id")
        .bind(State::Deleted)
        .fetch_all(conn)
        .await

}

pub async fn fetch_folder(conn: &mut PoolConnection<Sqlite>, local_id: LocalId) -> Result<Option<Folder>, Error> {
    sqlx::query_as("select * from folders where id = ?")
        .bind(local_id.0)
        .fetch_optional(conn)
        .await
}

pub async fn fetch_folder_by_remote_id(conn: &mut PoolConnection<Sqlite>, remote_id: RemoteId, account_id: i32) -> Result<Option<Folder>, Error> {
    sqlx::query_as("select * from folders where remote_id = ? and account_id = ?")
        .bind(remote_id.0)
        .bind(account_id)
        .fetch_optional(conn)
        .await
}

pub async fn create_folder(conn: &mut PoolConnection<Sqlite>, remote_id: Option<RemoteId>, account_id: i32, name: String) -> Result<Folder, Error> {
    conn.transaction(|conn| Box::pin(async move {
        sqlx::query("insert into folders (remote_id, account_id, name) values(?, ?, ?)")
            .bind(remote_id.map(|id| id.0))
            .bind(account_id)
            .bind(name.as_str())
            .execute(&mut *conn)
            .await
            .map(|_| ())?;

        sqlx::query_as("select * from folders order by id desc")
            .fetch_optional(conn)
            .await
            .map(|opt| opt.unwrap())
    }))
     .await
}

pub async fn delete_folder(conn: &mut PoolConnection<Sqlite>, local_id: LocalId) -> Result<(), Error> {
    sqlx::query("delete from folders where id = ?")
        .bind(local_id.0)
        .execute(&mut *conn)
        .await
        .map(|_| ())
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
}

pub async fn delete_folder_by_remote_id(conn: &mut PoolConnection<Sqlite>, remote_id: RemoteId, account_id: i32) -> Result<(), Error> {
    sqlx::query("delete from folders where remote_id = ? and account_id = ?")
        .bind(remote_id.0)
        .bind(account_id)
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn fetch_all_notes(conn: &mut PoolConnection<Sqlite>, folder_id: LocalId) -> Result<Vec<Note>, Error> {
    sqlx::query_as("select * from notes where folder_id = ? order by id")
        .bind(folder_id.0)
        .fetch_all(conn)
        .await
}

pub async fn fetch_notes(conn: &mut PoolConnection<Sqlite>, folder_id: LocalId) -> Result<Vec<Note>, Error> {
    sqlx::query_as("select * from notes where folder_id = ? and state != ? order by id")
        .bind(folder_id.0)
        .bind(State::Deleted)
        .fetch_all(conn)
        .await
}

pub async fn fetch_note(conn: &mut PoolConnection<Sqlite>, note_id: LocalId) -> Result<Option<Note>, Error> {
    sqlx::query_as("select * from notes where id = ?")
        .bind(note_id.0)
        .fetch_optional(conn)
        .await
}

pub async fn fetch_note_by_remote_id(conn: &mut PoolConnection<Sqlite>, note_id: RemoteId, folder_id: LocalId) -> Result<Option<Note>, Error> {
    sqlx::query_as("select * from notes where remote_id = ? and folder_id = ?")
        .bind(note_id.0)
        .bind(folder_id.0)
        .fetch_optional(conn)
        .await
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

        sqlx::query_as("select * from notes order by id desc")
            .fetch_optional(conn)
            .await
            .map(|opt| opt.unwrap())
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
}

pub async fn update_commit(conn: &mut PoolConnection<Sqlite>, note_id: LocalId, commit_id: i32) -> Result<(), Error> {
    sqlx::query("update notes set commit_id=?, state=? where id=?")
        .bind(commit_id)
        .bind(State::Clean)
        .bind(note_id.0)
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn delete_note(conn: &mut PoolConnection<Sqlite>, local_id: LocalId) -> Result<(), Error> {
    sqlx::query("delete from notes where id = ?")
        .bind(local_id.0)
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn delete_note_local(conn: &mut PoolConnection<Sqlite>, local_id: LocalId) -> Result<(), Error> {
    sqlx::query("update notes set state = ? where id = ?")
        .bind(State::Deleted)
        .bind(local_id.0)
        .execute(&mut *conn)
        .await
        .map(|_| ())
}
