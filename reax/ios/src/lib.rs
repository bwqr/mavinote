use std::{os::raw::{c_char, c_int, c_uchar}, ffi::{CStr, c_void}, sync::{Mutex, mpsc::Sender, Arc}, future::Future, str::FromStr};

use base::{Store, Config};
use once_cell::sync::OnceCell;
use reqwest::{header::{HeaderMap, HeaderValue}, ClientBuilder, Client};
use serde::Serialize;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite};
use tokio::task::JoinHandle;

mod note;
mod auth;

static ASYNC_RUNTIME: OnceCell<tokio::runtime::Runtime> = OnceCell::new();
static HANDLER: OnceCell<Mutex<Sender<(i32, bool, Vec<u8>)>>> = OnceCell::new();
static DATABASE: OnceCell<Pool<Sqlite>> = OnceCell::new();

#[derive(Serialize)]
enum Message<T: Serialize> {
    Ok(T),
    Err(base::Error),
    Complete,
}

pub(crate) fn send_stream<T: Serialize>(stream_id: i32, message: Message<T>) {
    let bytes = bincode::serialize(&message).expect("failed to searialize message");

    HANDLER
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .send((stream_id, true, bytes))
        .unwrap();
}

pub(crate) fn send_once<T: Serialize>(once_id: i32, message: Result<T, base::Error>) {
    let bytes = bincode::serialize(&message).expect("failed to searialize message");

    HANDLER
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .send((once_id, false, bytes))
        .unwrap();
}


pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    ASYNC_RUNTIME.get().unwrap().spawn(future)
}

#[no_mangle]
pub extern fn reax_init(api_url: *const c_char, storage_dir: *const c_char) {
    let api_url = unsafe { CStr::from_ptr(api_url).to_str().unwrap().to_string() };
    let storage_dir = unsafe { CStr::from_ptr(storage_dir).to_str().unwrap().to_string() };

    std::env::set_var("RUST_LOG", "debug,sqlx=warn");
    env_logger::init();

    ASYNC_RUNTIME
        .set(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to initialize tokio runtime"),
        )
        .expect("failed to set tokio runtime");

    runtime::init();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    runtime::put::<Arc<Client>>(Arc::new(client));

    let db_path = format!("sqlite:{}/app.db", storage_dir);
    let pool = ASYNC_RUNTIME.get().unwrap().block_on(async move {
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
    DATABASE.set(pool.clone()).unwrap();

    runtime::put::<Arc<dyn Store>>(Arc::new(util::store::FileStore::new(pool)));

    runtime::put::<Arc<Config>>(Arc::new(Config {
        api_url,
        storage_dir,
    }));

    ::note::init();

    ::log::info!("reax runtime is initialized");
}

#[no_mangle]
pub extern fn reax_init_handler(ptr: *const c_void, f: unsafe extern fn(c_int, c_uchar, *const c_uchar, c_int, *const c_void)) {
    let (send, recv) = std::sync::mpsc::channel();

    HANDLER
        .set(Mutex::new(send))
        .map_err(|_| "HandlerError")
        .expect("failed to set handler");

    while let Ok((wait_id, ok, bytes)) = recv.recv() {
        unsafe { f(wait_id, ok as c_uchar, bytes.as_ptr() as *const c_uchar, bytes.len() as c_int, ptr) }
    }
}

#[no_mangle]
pub extern "C" fn reax_abort(pointer: * mut c_void) {
    let handle = unsafe { Box::from_raw(pointer as * mut tokio::task::JoinHandle<()>) };

    ::log::debug!("received abort, {:p}", handle);

    handle.abort();
}
