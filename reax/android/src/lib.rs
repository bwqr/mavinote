#![allow(non_snake_case)]

use std::ffi::CString;

use jni::{JNIEnv, objects::{JObject, JString}, sys::jint};

mod log;

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
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1folders(env: JNIEnv, _: JObject) -> jni::sys::jbyteArray {
    let folders = runtime::block_on(async move {
        note::folders(runtime::client(), runtime::config()).await
    });

    let bytes = bincode::serialize(&folders).expect("failed to serialize val");

    let bytes_array = env.new_byte_array(bytes.len().try_into().unwrap()).unwrap();
    env.set_byte_array_region(bytes_array, 0, bytes.iter().map(|byte| *byte as i8).collect::<Vec<i8>>().as_slice()).unwrap();

    bytes_array
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1addFolder(env: JNIEnv, _: JObject, name: JString) {
    let name = env.get_string(name).unwrap().to_str().unwrap().to_owned();

    runtime::block_on(async move {
        note::create_folder(runtime::client(), runtime::config(), name).await
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1noteSummaries(env: JNIEnv, _: JObject, folder_id: jint) -> jni::sys::jbyteArray {
    let summaries = runtime::block_on(async move {
        note::note_summaries(runtime::client(), runtime::config(), folder_id).await
    });

    let bytes = bincode::serialize(&summaries).expect("failed to serialize val");

    let bytes_array = env.new_byte_array(bytes.len().try_into().unwrap()).unwrap();
    env.set_byte_array_region(bytes_array, 0, bytes.iter().map(|byte| *byte as i8).collect::<Vec<i8>>().as_slice()).unwrap();

    bytes_array
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1note(env: JNIEnv, _: JObject, note_id: jint) -> jni::sys::jbyteArray {
    let note = runtime::block_on(async move {
        note::note(runtime::client(), runtime::config(), note_id).await
    });

    match note {
        None => env.new_byte_array(0).unwrap(),
        Some(note) => {
            let bytes = bincode::serialize(&note).expect("failed to serialize val");

            let bytes_array = env.new_byte_array(bytes.len().try_into().unwrap()).unwrap();
            env.set_byte_array_region(bytes_array, 0, bytes.iter().map(|byte| *byte as i8).collect::<Vec<i8>>().as_slice()).unwrap();

            bytes_array
        },
    }
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1createNote(_: JNIEnv, _: JObject, folder_id: jint) -> jint {
    runtime::block_on(async move {
        note::create_note(runtime::client(), runtime::config(), folder_id).await
    })
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1updateNote(env: JNIEnv, _: JObject, note_id: jint, text: JString) {
    let text = env.get_string(text).unwrap().to_str().unwrap().to_owned();

    runtime::block_on(async move {
        note::update_note(runtime::client(), runtime::config(), note_id, text).await
    });
}
