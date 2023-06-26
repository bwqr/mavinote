use std::{sync::{OnceLock, Mutex, mpsc::Sender, Arc}, future::Future};

use serde::Serialize;
use tokio::task::JoinHandle;

use base::Config;

pub mod account;
pub mod note;

static ASYNC_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
static HANDLER: OnceLock<Mutex<Sender<(i32, bool, Vec<u8>)>>> = OnceLock::new();

#[derive(Serialize)]
enum Message<T: Serialize, E: Serialize> {
    Ok(T),
    Err(E),
    Complete,
}

pub fn init(
    api_url: String,
    ws_url: String,
    storage_dir: String,
) {
    ASYNC_RUNTIME
        .set(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to initialize tokio runtime"),
        )
        .expect("failed to set tokio runtime");

    runtime::init();

    runtime::put::<Arc<Config>>(Arc::new(Config {
        api_url,
        ws_url,
        storage_dir,
    }));
}

pub fn init_handler(handler: Sender<(i32, bool, Vec<u8>)>) {
    HANDLER
        .set(Mutex::new(handler))
        .map_err(|_| "HandlerError")
        .expect("failed to set handler");
}

pub(crate) fn send_stream<T: Serialize, E: Serialize>(stream_id: i32, message: Message<T, E>) {
    let bytes = bincode::serialize(&message).expect("failed to searialize message");

    HANDLER
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .send((stream_id, true, bytes))
        .unwrap();
}

pub(crate) fn send_once<T: Serialize, E: Serialize>(once_id: i32, message: Result<T, E>) {
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

pub fn block_on<F: Future>(future: F) -> F::Output {
    ASYNC_RUNTIME.get().unwrap().block_on(future)
}
