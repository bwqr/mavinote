#![allow(non_snake_case)]

use std::{ffi::CString, sync::{mpsc::Sender, Mutex}};

use jni::{JNIEnv, objects::{JObject, JString, JValue}, signature::{JavaType, Primitive}};
use once_cell::sync::OnceCell;
use serde::Serialize;

mod auth;
mod log;
mod note;

static HANDLER: OnceCell<Mutex<Sender<(i32, bool, Vec<u8>)>>> = OnceCell::new();

pub(crate) fn send<T: Serialize, E: Serialize>(wait_id: i32, res: Result<T, E>) {
    let bytes = match &res {
        Ok(value) => bincode::serialize(value).expect("failed to serialize val"),
        Err(e) => bincode::serialize(e).expect("failed to serialize val"),
    };

    HANDLER.get().unwrap().lock().unwrap().send((wait_id, res.is_ok(), bytes)).unwrap();
}

fn capture_stderr() {
    std::thread::spawn(|| {
        unsafe {
            let mut pipes: [i32; 2] = [0; 2];
            libc::pipe(&mut pipes as *mut i32);
            libc::dup2(pipes[1], libc::STDERR_FILENO);

            let readonly = CString::new("r").unwrap();
            let file = libc::fdopen(pipes[0], readonly.as_ptr());

            let mut buff:[i8; 256] = [0; 256];
            let tag = CString::new("stderr").unwrap();

            loop {
                libc::fgets(&mut buff as *mut i8, 256, file);
                log::__android_log_write(2, tag.as_ptr(), buff.as_ptr());
            }
        }
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_Runtime__1init(env: JNIEnv, _: JObject, app_name: JString, api_url: JString, storage_dir: JString) {
    capture_stderr();

    let app_name = env.get_string(app_name).unwrap().to_str().unwrap().to_owned();
    let api_url = env.get_string(api_url).unwrap().to_str().unwrap().to_owned();
    let storage_dir = env.get_string(storage_dir).unwrap().to_str().unwrap().to_owned();

    std::env::set_var("RUST_LOG", "debug");
    log::init(app_name);

    runtime::init(base::Config { api_url, storage_dir });

    ::log::info!("reax runtime is initialized");
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_Runtime__1initHandler(env: JNIEnv, _: JObject, callback: JObject) {
    let (send, recv) = std::sync::mpsc::channel();
    HANDLER
        .set(Mutex::new(send))
        .map_err(|_| "HandlerError")
        .expect("failed to set handler");

    let callback_class = env.get_object_class(callback).unwrap();
    let callback_method_id = env.get_method_id(callback_class, "invoke", "(IZ[B)V").unwrap();

    while let Ok((wait_id, ok, bytes)) = recv.recv() {
        let bytes_array = env.new_byte_array(bytes.len().try_into().unwrap()).unwrap();
        env.set_byte_array_region(bytes_array, 0, bytes.iter().map(|byte| *byte as i8).collect::<Vec<i8>>().as_slice()).unwrap();

        if let Err(e) = env.call_method_unchecked(callback, callback_method_id, JavaType::Primitive(Primitive::Void), &[JValue::Int(wait_id), JValue::Bool(ok as u8), JValue::Object(bytes_array.into())]) {
            ::log::error!("failed to call storeHandler, {:?}", e);
        }

        if let Ok(true) = env.exception_check() {
            ::log::error!("exception is occured");
            env.exception_describe().unwrap();
            env.exception_clear().unwrap();
        }
    }
}
