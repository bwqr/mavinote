use sqlx::Row;

#[derive(Debug)]
pub struct Store;

impl Store {
    pub fn new() -> Self {
        Self
    }

    pub async fn get(key: &str) -> Result<Option<String>, sqlx::Error> {
        let mut conn = crate::pool().acquire().await?;

        let row = sqlx::query("select value from store where key = ?")
            .bind(key)
            .fetch_optional(&mut conn)
            .await?;

        if let Some(row) = row {
            row.try_get("value")
        } else {
            Ok(None)
        }
    }

    pub async fn put(key: &str, value: &str) -> Result<(), sqlx::Error> {
        let mut conn = crate::pool().acquire().await?;

        let res = sqlx::query("insert into store values (?, ?)")
            .bind(key)
            .bind(value)
            .execute(&mut conn)
            .await
            .map(|_| ());

        if let Err(sqlx::Error::Database(e)) = &res {
            if let Some("2067") = e.code().as_deref() {
                sqlx::query("update store set value = ? where key = ?")
                    .bind(key)
                    .bind(value)
                    .execute(&mut conn)
                    .await?;

                return Ok(())
            }
        }

        res
    }
}
