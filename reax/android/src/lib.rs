#![allow(non_snake_case)]

use jni::{JNIEnv, objects::{JObject, JString}, sys::jint};

mod log;

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_Runtime__1init(env: JNIEnv, _: JObject, app_name: JString, api_url: JString, storage_dir: JString) {
    let app_name = env.get_string(app_name).unwrap().to_str().unwrap().to_owned();
    let api_url = env.get_string(api_url).unwrap().to_str().unwrap().to_owned();
    let storage_dir = env.get_string(storage_dir).unwrap().to_str().unwrap().to_owned();

    log::init(app_name);

    runtime::init(runtime::Config { api_url, storage_dir });

    ::log::info!("reax runtime is initialized");
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1folders(env: JNIEnv, _: JObject) -> jni::sys::jbyteArray {
    let folders = runtime::block_on(async move {
        let conn = runtime::pool().acquire().await.unwrap();

        note::folders(conn).await
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
        let conn = runtime::pool().acquire().await.unwrap();

        note::add_folder(conn, name).await
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1noteSummaries(env: JNIEnv, _: JObject, folder_id: jint) -> jni::sys::jbyteArray {
    let summaries = runtime::block_on(async move {
        let conn = runtime::pool().acquire().await.unwrap();

        note::note_summaries(conn, folder_id).await
    });

    let bytes = bincode::serialize(&summaries).expect("failed to serialize val");

    let bytes_array = env.new_byte_array(bytes.len().try_into().unwrap()).unwrap();
    env.set_byte_array_region(bytes_array, 0, bytes.iter().map(|byte| *byte as i8).collect::<Vec<i8>>().as_slice()).unwrap();

    bytes_array
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1note(env: JNIEnv, _: JObject, note_id: jint) -> jni::sys::jbyteArray {
    let note = runtime::block_on(async move {
        let conn = runtime::pool().acquire().await.unwrap();

        note::note(conn, note_id).await
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
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1addNote(env: JNIEnv, _: JObject, folder_id: jint) -> jint {
    runtime::block_on(async move {
        let conn = runtime::pool().acquire().await.unwrap();

        note::add_note(conn, folder_id).await
    })
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1updateNote(env: JNIEnv, _: JObject, note_id: jint, text: JString) {
    let text = env.get_string(text).unwrap().to_str().unwrap().to_owned();

    runtime::block_on(async move {
        let conn = runtime::pool().acquire().await.unwrap();

        note::update_note(conn, note_id, text).await
    });
}
