use std::{sync::{OnceLock, Mutex, mpsc::Sender, Arc}, future::Future, str::FromStr};

use serde::Serialize;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite};
use tokio::task::JoinHandle;

use base::Config;

pub mod account;
pub mod note;

static ASYNC_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
static HANDLER: OnceLock<Mutex<Sender<(i32, Vec<u8>)>>> = OnceLock::new();

#[derive(Serialize)]
enum Message<T: Serialize> {
    Value(T),
    Complete,
}

pub fn init(
    api_url: String,
    ws_url: String,
    storage_dir: String,
) -> Vec<u8> {
    // Returns true if the init is called for the first time.
    // If init was called previously, it would return false
    let init_app = move || -> Result<(), String> {
        ASYNC_RUNTIME
        .set(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to initialize tokio runtime"),
        )
        .expect("failed to set tokio runtime");

        runtime::init();

        let db_path = format!("sqlite:{}/app.db", &storage_dir);

        let pool = block_on(async move {
            let options = SqliteConnectOptions::from_str(&db_path)
                .map_err(|e| format!("Failed to create SqliteConnectOptions from path, {e:?}"))?
                .create_if_missing(true);

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect_with(options)
                .await
                .map_err(|e| format!("Failed to create sqlite connection pool, {e:?}"))?;

            sqlx::migrate!("../migrations")
                .run(&pool)
                .await
                .map_err(|e| format!("Failed to apply database migrations, {e:?}"))?;

            Result::<Pool<Sqlite>, String>::Ok(pool)
        })?;

        runtime::put::<Arc<Pool<Sqlite>>>(Arc::new(pool));

        runtime::put::<Arc<Config>>(Arc::new(Config {
            api_url,
            ws_url,
            storage_dir,
        }));

        ::log::info!("reax is built with {} profile", if cfg!(debug_assertions) { "debug" } else { "release" });
        ::log::info!("reax runtime is initialized");

        Ok(())
    };

    bincode::serialize(&init_app()).unwrap()
}

pub fn init_handler(handler: Sender<(i32, Vec<u8>)>) {
    HANDLER
        .set(Mutex::new(handler))
        .map_err(|_| "HandlerError")
        .expect("failed to set handler");
}

pub unsafe fn abort(pointer: * mut tokio::task::JoinHandle<()>) {
    let handle = unsafe { Box::from_raw(pointer) };

    ::log::debug!("received abort, {:p}", handle);

    handle.abort();
}

pub(crate) fn send_stream<T: Serialize>(stream_id: i32, message: Message<T>) {
    let bytes = bincode::serialize(&message).expect("failed to searialize message");

    HANDLER
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .send((stream_id, bytes))
        .unwrap();
}

pub(crate) fn send_once<T: Serialize, E: Serialize>(once_id: i32, message: Result<T, E>) {
    let bytes = bincode::serialize(&message).expect("failed to searialize message");

    HANDLER
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .send((once_id, bytes))
        .unwrap();
}

pub(crate) fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    ASYNC_RUNTIME.get().unwrap().spawn(future)
}

pub(crate) fn block_on<F: Future>(future: F) -> F::Output {
    ASYNC_RUNTIME.get().unwrap().block_on(future)
}
