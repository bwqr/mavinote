use std::{pin::Pin, future::Future};

use sqlx::{Row, Pool, Sqlite};

use base::{Store, Error};

pub struct FileStore {
    pool: Pool<Sqlite>,
}

impl FileStore {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl Store for FileStore {
    fn get<'a>(&'a self, key: &'a str) -> Pin<Box<dyn Future<Output = Result<Option<String>, Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut conn = self.pool.acquire().await.map_err(|_| Error::Database)?;

            let row = sqlx::query("select value from store where key = ?")
                .bind(key)
                .fetch_optional(&mut conn)
                .await
                .map_err(|_| Error::Database)?;

            if let Some(row) = row {
                row.try_get("value")
                    .map_err(|_| Error::Database)
            } else {
                Ok(None)
            }
        })
    }

    fn put<'a>(&'a self, key: &'a str, value: &'a str) -> Pin<Box<dyn Future<Output = Result<(), base::Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut conn = self.pool.acquire().await.map_err(|_| Error::Database)?;

            let res = sqlx::query("insert into store values (?, ?)")
                .bind(key)
                .bind(value)
                .execute(&mut conn)
                .await
                .map(|_| ());

            if let Err(sqlx::Error::Database(e)) = &res {
                if let Some("2067") = e.code().as_deref() {
                    return sqlx::query("update store set value = ? where key = ?")
                        .bind(value)
                        .bind(key)
                        .execute(&mut conn)
                        .await
                        .map(|_| ())
                        .map_err(|_| Error::Database)
                }
            }

            res
                .map_err(|_| Error::Database)
        })
    }
}
