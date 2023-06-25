#![allow(non_snake_case)]

use std::{
    str::FromStr,
    ffi::CString,
    sync::Arc,
};

use jni::{
    objects::{JObject, JString, JValue, JClass},
    signature::{JavaType, Primitive},
    JNIEnv, sys::jlong,
};
use libc::c_char;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite};

mod account;
mod log;
mod note;

fn capture_stderr() {
    std::thread::spawn(|| unsafe {
        let mut pipes: [i32; 2] = [0; 2];
        libc::pipe(&mut pipes as *mut i32);
        libc::dup2(pipes[1], libc::STDERR_FILENO);

        let readonly = CString::new("r").unwrap();
        let file = libc::fdopen(pipes[0], readonly.as_ptr());

        let mut buff: [c_char; 256] = [0; 256];
        let tag = CString::new("stderr").unwrap();

        loop {
            libc::fgets(&mut buff as *mut c_char, 256, file);
            log::__android_log_write(5, tag.as_ptr(), buff.as_ptr());
        }
    });
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_reax_RuntimeKt__1init(
    env: JNIEnv,
    _: JClass,
    api_url: JString,
    ws_url: JString,
    storage_dir: JString,
) {
    capture_stderr();

    let api_url = env
        .get_string(api_url)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let ws_url = env
        .get_string(ws_url)
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

    universal::init(api_url, ws_url, storage_dir.clone());

    let db_path = format!("sqlite:{}/app.db", storage_dir);
    let pool = universal::block_on(async move {
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

    ::log::info!("reax is built with {} profile", if cfg!(debug_assertions) { "debug" } else { "release" });
    ::log::info!("reax runtime is initialized");
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_reax_RuntimeKt__1initHandler(
    env: JNIEnv,
    _: JClass,
    callback: JObject,
) {
    let (send, recv) = std::sync::mpsc::channel();
    universal::init_handler(send);

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
pub extern "C" fn Java_com_bwqr_mavinote_reax_RuntimeKt__1abort(
    _: JNIEnv,
    _: JClass,
    pointer: jlong,
) {
    let handle = unsafe { Box::from_raw(pointer as * mut tokio::task::JoinHandle<()>) };

    ::log::debug!("received abort, {:p}", handle);

    handle.abort();
}
