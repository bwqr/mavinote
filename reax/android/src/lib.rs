#![allow(non_snake_case)]

use std::{ffi::CString, sync::{mpsc::Sender, Mutex}};

use jni::{JNIEnv, objects::{JObject, JString, JValue}, sys::jint, signature::{JavaType, Primitive}};
use once_cell::sync::OnceCell;

mod log;

static HANDLER: OnceCell<Mutex<Sender<(i32, Vec<u8>)>>> = OnceCell::new();

fn send(wait_id: i32, bytes: Vec<u8>) {
    HANDLER.get().unwrap().lock().unwrap().send((wait_id, bytes)).unwrap();
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
    let callback_method_id = env.get_method_id(callback_class, "invoke", "(I[B)V").unwrap();

    while let Ok((wait_id, bytes)) = recv.recv() {
        let bytes_array = env.new_byte_array(bytes.len().try_into().unwrap()).unwrap();
        env.set_byte_array_region(bytes_array, 0, bytes.iter().map(|byte| *byte as i8).collect::<Vec<i8>>().as_slice()).unwrap();

        if let Err(e) = env.call_method_unchecked(callback, callback_method_id, JavaType::Primitive(Primitive::Void), &[JValue::Int(wait_id), JValue::Object(bytes_array.into())]) {
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
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1folders(_: JNIEnv, _: JObject, wait_id: jint) {
    runtime::spawn(async move {
        let folders = note::folders(runtime::client(), runtime::config()).await;

        let bytes = bincode::serialize(&folders).expect("failed to serialize val");

        send(wait_id, bytes);
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1addFolder(env: JNIEnv, _: JObject, wait_id: jint, name: JString) {
    let name = env.get_string(name).unwrap().to_str().unwrap().to_owned();

    runtime::spawn(async move {
        note::create_folder(runtime::client(), runtime::config(), name).await;

        send(wait_id, vec![]);
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1noteSummaries(_: JNIEnv, _: JObject, wait_id: jint, folder_id: jint) {
    runtime::spawn(async move {
        let summaries = note::note_summaries(runtime::client(), runtime::config(), folder_id).await;

        let bytes = bincode::serialize(&summaries).expect("failed to serialize val");

        send(wait_id, bytes);
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1note(_: JNIEnv, _: JObject, wait_id: jint, note_id: jint) {
    runtime::spawn(async move {
        let note = note::note(runtime::client(), runtime::config(), note_id).await;

        let bytes = match note {
            None => vec![],
            Some(note) => bincode::serialize(&note).expect("failed to serialize val")
        };

        send(wait_id, bytes);
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1createNote(_: JNIEnv, _: JObject, wait_id: jint, folder_id: jint) {
    runtime::spawn(async move {
        let note_id = note::create_note(runtime::client(), runtime::config(), folder_id).await;

        let bytes = bincode::serialize(&note_id).expect("failed to serialize val");

        send(wait_id, bytes);
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1updateNote(env: JNIEnv, _: JObject, wait_id: jint, note_id: jint, text: JString) {
    let text = env.get_string(text).unwrap().to_str().unwrap().to_owned();

    runtime::spawn(async move {
        note::update_note(runtime::client(), runtime::config(), note_id, text).await;

        send(wait_id, vec![]);
    });
}
