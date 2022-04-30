pub mod models;

use sqlx::{Sqlite, pool::PoolConnection, Connection};

use models::{Folder, Note};

pub async fn folders(mut conn: PoolConnection<Sqlite>) -> Vec<Folder> {
    sqlx::query_as("select * from folders order by id")
        .fetch_all(&mut conn)
        .await
        .unwrap()
}

pub async fn add_folder(mut conn: PoolConnection<Sqlite>, name: String) {
    sqlx::query("insert into folders (name) values (?)")
        .bind(name)
        .execute(&mut conn)
        .await
        .unwrap();
}

pub async fn note_summaries(mut conn: PoolConnection<Sqlite>, folder_id: i32) -> Vec<Note> {
    sqlx::query_as("select * from notes where folder_id = ? order by id")
        .bind(folder_id)
        .fetch_all(&mut conn)
        .await
        .unwrap()
}

pub async fn note(mut conn: PoolConnection<Sqlite>, note_id: i32) -> Option<Note> {
    sqlx::query_as("select * from notes where id = ? order by id")
        .bind(note_id)
        .fetch_optional(&mut conn)
        .await
        .unwrap()
}

pub async fn add_note(mut conn: PoolConnection<Sqlite>, folder_id: i32) -> i32 {
    conn.transaction(|conn| Box::pin(async move {
        sqlx::query("insert into notes (folder_id, title, text) values (?, ?, ?)")
            .bind(folder_id)
            .bind("Newly created note")
            .bind("")
            .execute(&mut *conn)
            .await?;

        sqlx::query_as::<_, (i32,)>("select id from notes order by id desc")
            .fetch_one(conn)
            .await
            .map(|row| row.0)
    }))
        .await
        .unwrap()
}

pub async fn update_note(mut conn: PoolConnection<Sqlite>, note_id: i32, text: String) {
    sqlx::query("update notes set title = ?, text = ? where id = ?")
        .bind(&text.as_str()[..text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0])
        .bind(&text)
        .bind(note_id)
        .execute(&mut conn)
        .await
        .unwrap();
}
