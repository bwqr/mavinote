use serde::de::DeserializeOwned;
use sqlx::Connection;
use sqlx::types::Json;
use sqlx::{Sqlite, pool::PoolConnection, Error};

use crate::models::{Folder, Note, State, RemoteId, LocalId, Account, AccountKind, Mavinote, Device};

pub async fn fetch_accounts(conn: &mut PoolConnection<Sqlite>) -> Result<Vec<Account>, Error> {
    sqlx::query_as("select id, name, kind from accounts order by id")
        .fetch_all(conn)
        .await
}

pub async fn fetch_account(conn: &mut PoolConnection<Sqlite>, account_id: i32) -> Result<Option<Account>, Error> {
    sqlx::query_as("select id, name, kind from accounts where id = ?")
        .bind(account_id)
        .fetch_optional(conn)
        .await
}

pub async fn fetch_account_data<T: DeserializeOwned + Unpin + Send + 'static>(conn: &mut PoolConnection<Sqlite>, account_id: i32) -> Result<Option<T>, Error> {
    sqlx::query_as::<Sqlite, (Json<T>,)>("select data from accounts where id = ?")
        .bind(account_id)
        .fetch_optional(conn)
        .await
        .map(|opt| opt.map(|json| json.0.0))
}

pub async fn fetch_devices(conn: &mut PoolConnection<Sqlite>, account_id: i32) -> Result<Vec<Device>, Error> {
    sqlx::query_as("select id, account_id from devices where account_id = ?")
        .bind(account_id)
        .fetch_all(conn)
        .await
}

pub async fn create_devices(conn: &mut PoolConnection<Sqlite>, account_id: i32, device_ids: &[i32]) -> Result<(), Error> {
    let device_ids: String = itertools::Itertools::intersperse(device_ids.into_iter().map(|id| format!("({}, {})", id, account_id)), ",".to_string()).collect();
    let query = format!("insert into devices (id, account_id) values {}", device_ids);

    let query = sqlx::query(&query);

    query.execute(&mut *conn)
        .await
        .map(|_| ())
}

pub async fn delete_devices(conn: &mut PoolConnection<Sqlite>, account_id: i32) -> Result<(), Error> {
    sqlx::query("delete from devices where account_id = ?")
        .bind(account_id)
        .execute(&mut *conn)
        .await
        .map(|_| ())
}

pub async fn account_with_name_exists(conn: &mut PoolConnection<Sqlite>, name: &str) -> Result<bool, Error> {
    sqlx::query_as::<Sqlite, (i32,)>("select id from accounts where name = ?")
        .bind(name)
        .fetch_optional(conn)
        .await
        .map(|opt| opt.is_some())
}

pub async fn fetch_account_folders(conn: &mut PoolConnection<Sqlite>, account_id: i32) -> Result<Vec<Folder>, Error> {
    sqlx::query_as("select * from folders where account_id = ? order by id")
        .bind(account_id)
        .fetch_all(conn)
        .await
}

pub async fn create_account(conn: &mut PoolConnection<Sqlite>, name: String, kind: AccountKind, data: Option<Json<Mavinote>>) -> Result<Account, Error> {
    conn.transaction(|conn| Box::pin(async move {
        sqlx::query("insert into accounts (name, kind, data) values(?, ?, ?)")
            .bind(name)
            .bind(kind)
            .bind(data)
            .execute(&mut *conn)
            .await
            .map(|_| ())?;

        sqlx::query_as("select id, name, kind from accounts order by id desc")
            .fetch_optional(conn)
            .await
            .map(|opt| opt.unwrap())
    }))
     .await
}

pub async fn update_account_data(conn: &mut PoolConnection<Sqlite>, account_id: i32, data: Option<Json<Mavinote>>) -> Result<(), Error> {
    sqlx::query("update accounts set data = ? where id = ?")
        .bind(data)
        .bind(account_id)
        .execute(&mut *conn)
        .await
        .map(|_| ())
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

pub async fn update_folder_remote_id(conn: &mut PoolConnection<Sqlite>, local_id: LocalId, remote_id: RemoteId) -> Result<(), Error> {
    sqlx::query("update folders set remote_id = ? where id = ?")
        .bind(remote_id.0)
        .bind(local_id.0)
        .execute(&mut *conn)
        .await
        .map(|_| ())
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

pub async fn create_note(conn: &mut PoolConnection<Sqlite>, folder_id: LocalId, remote_id: Option<RemoteId>, name: String, text: String, commit: i32) -> Result<Note, Error> {
    conn.transaction(|conn| Box::pin(async move {
        sqlx::query("insert into notes (folder_id, remote_id, name, text, 'commit', state) values(?, ?, ?, ?, ?, ?)")
            .bind(folder_id.0)
            .bind(remote_id.map(|id| id.0))
            .bind(name.as_str())
            .bind(text.as_str())
            .bind(commit)
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

pub async fn update_note(conn: &mut PoolConnection<Sqlite>, note_id: LocalId, name: &str, text: &str, commit: i32, state: State) -> Result<(), Error> {
    sqlx::query("update notes set name=?, text=?, 'commit'=?, state=? where id=?")
        .bind(name)
        .bind(text)
        .bind(commit)
        .bind(state)
        .bind(note_id.0)
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn update_commit(conn: &mut PoolConnection<Sqlite>, note_id: LocalId, commit: i32) -> Result<(), Error> {
    sqlx::query("update notes set commit=?, state=? where id=?")
        .bind(commit)
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
