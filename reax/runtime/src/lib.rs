use std::str::FromStr;
use std::future::Future;

use reqwest::Client;
use sqlx::{Sqlite, Pool, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use once_cell::sync::OnceCell;
use tokio::task::JoinHandle;

use base::Config;

static ASYNC_RUNTIME: OnceCell<tokio::runtime::Runtime> = OnceCell::new();
static CONFIG: OnceCell<Config> = OnceCell::new();
static DATABASE: OnceCell<Pool<Sqlite>> = OnceCell::new();
static CLIENT: OnceCell<Client> = OnceCell::new();

pub fn init(config: Config) {
    ASYNC_RUNTIME
        .set(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to initialize tokio runtime"),
        )
        .expect("failed to set tokio runtime");

    let db_path = format!("sqlite:{}/app.db", config.storage_dir);
    let pool = block_on(async move {
        let options = SqliteConnectOptions::from_str(db_path.as_str())
            .unwrap()
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .unwrap();

        sqlx::migrate!("../migrations").run(&pool).await.unwrap();

        pool
    });

    DATABASE.set(pool).expect("failed to set database");

    CLIENT.set(Client::new()).expect("failed to set client");

    CONFIG.set(config).expect("failed to set config");
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    ASYNC_RUNTIME.get().unwrap().spawn(future)
}

pub fn block_on<F>(future: F) -> F::Output
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    ASYNC_RUNTIME.get().unwrap().block_on(future)
}

pub fn pool() -> &'static Pool<Sqlite> {
    DATABASE.get().unwrap()
}

pub fn client() -> &'static Client {
    CLIENT.get().unwrap()
}

pub fn config() -> &'static Config {
    CONFIG.get().unwrap()
}
