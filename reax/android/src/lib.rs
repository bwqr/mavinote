#![allow(non_snake_case)]

use std::{
    str::FromStr,
    ffi::CString,
    sync::{mpsc::Sender, Mutex, Arc}, future::Future,
};

use base::{Config, Store};
use jni::{
    objects::{JObject, JString, JValue},
    signature::{JavaType, Primitive},
    JNIEnv, sys::jlong,
};
use once_cell::sync::OnceCell;
use reqwest::{header::{HeaderMap, HeaderValue}, ClientBuilder, Client};
use serde::Serialize;
use tokio::task::JoinHandle;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite};

mod log;
mod note;
mod notify;

static ASYNC_RUNTIME: OnceCell<tokio::runtime::Runtime> = OnceCell::new();
static HANDLER: OnceCell<Mutex<Sender<(i32, bool, Vec<u8>)>>> = OnceCell::new();

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

fn capture_stderr() {
    std::thread::spawn(|| unsafe {
        let mut pipes: [i32; 2] = [0; 2];
        libc::pipe(&mut pipes as *mut i32);
        libc::dup2(pipes[1], libc::STDERR_FILENO);

        let readonly = CString::new("r").unwrap();
        let file = libc::fdopen(pipes[0], readonly.as_ptr());

        let mut buff: [i8; 256] = [0; 256];
        let tag = CString::new("stderr").unwrap();

        loop {
            libc::fgets(&mut buff as *mut i8, 256, file);
            log::__android_log_write(5, tag.as_ptr(), buff.as_ptr());
        }
    });
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_Runtime__1init(
    env: JNIEnv,
    _: JObject,
    api_url: JString,
    notify_url: JString,
    storage_dir: JString,
) {
    capture_stderr();

    let api_url = env
        .get_string(api_url)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let notify_url = env
        .get_string(notify_url)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let storage_dir = env
        .get_string(storage_dir)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    log::init();

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

    runtime::put::<Arc<Pool<Sqlite>>>(Arc::new(pool.clone()));

    runtime::put::<Arc<dyn Store>>(Arc::new(util::store::FileStore::new(pool)));

    runtime::put::<Arc<Config>>(Arc::new(Config {
        api_url,
        storage_dir,
    }));

    ::note::init();
    ::notify::init(notify_url);
    ::log::info!("reax runtime is initialized");
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_Runtime__1initHandler(
    env: JNIEnv,
    _: JObject,
    callback: JObject,
) {
    let (send, recv) = std::sync::mpsc::channel();
    HANDLER
        .set(Mutex::new(send))
        .map_err(|_| "HandlerError")
        .expect("failed to set handler");

    let callback_class = env.get_object_class(callback).unwrap();
    let callback_method_id = env
        .get_method_id(callback_class, "invoke", "(IZ[B)V")
        .unwrap();

    while let Ok((wait_id, is_stream, bytes)) = recv.recv() {
        let bytes_array = env.new_byte_array(bytes.len().try_into().unwrap()).unwrap();
        env.set_byte_array_region(
            bytes_array,
            0,
            bytes
                .iter()
                .map(|byte| *byte as i8)
                .collect::<Vec<i8>>()
                .as_slice(),
        )
        .unwrap();

        if let Err(e) = env.call_method_unchecked(
            callback,
            callback_method_id,
            JavaType::Primitive(Primitive::Void),
            &[
                JValue::Int(wait_id),
                JValue::Bool(is_stream as u8),
                JValue::Object(bytes_array.into()),
            ],
        ) {
            ::log::error!("failed to call storeHandler, {:?}", e);
        }

        if let Ok(true) = env.exception_check() {
            ::log::error!("exception is occured");
            env.exception_describe().unwrap();
            env.exception_clear().unwrap();
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_Runtime__1abort(
    _: JNIEnv,
    _: JObject,
    pointer: jlong,
) {
    let handle = unsafe { Box::from_raw(pointer as * mut tokio::task::JoinHandle<()>) };

    ::log::debug!("received abort, {:p}", handle);

    handle.abort();
}
