use std::{os::raw::c_char, ffi::{CStr, c_void}, sync::Arc, str::FromStr};

use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite};

mod note;
mod account;

#[no_mangle]
pub extern fn reax_init(
    api_url: *const c_char,
    ws_url: *const c_char,
    storage_dir: *const c_char,
) {
    let api_url = unsafe { CStr::from_ptr(api_url).to_str().unwrap().to_string() };
    let ws_url = unsafe { CStr::from_ptr(ws_url).to_str().unwrap().to_string() };
    let storage_dir = unsafe { CStr::from_ptr(storage_dir).to_str().unwrap().to_string() };

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

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

    ::log::info!("reax runtime is initialized");
}

#[no_mangle]
pub extern fn reax_init_handler(ptr: * mut c_void, f: unsafe extern fn(i32, *const u8, usize, * mut c_void)) {
    let (send, recv) = std::sync::mpsc::channel();

    universal::init_handler(send);

    while let Ok((wait_id, bytes)) = recv.recv() {
        unsafe { f(wait_id, bytes.as_ptr(), bytes.len(), ptr) }
    }
}

#[no_mangle]
pub extern "C" fn reax_abort(pointer: * mut c_void) {
    let handle = unsafe { Box::from_raw(pointer as * mut tokio::task::JoinHandle<()>) };

    ::log::debug!("received abort, {:p}", handle);

    handle.abort();
}
