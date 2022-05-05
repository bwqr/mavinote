use jni::{JNIEnv, objects::{JObject, JString}, sys::jint};

use crate::send;

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1folders(_: JNIEnv, _: JObject, wait_id: jint) {
    runtime::spawn(async move {
        let res = note::folders(runtime::client(), runtime::config()).await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1addFolder(env: JNIEnv, _: JObject, wait_id: jint, name: JString) {
    let name = env.get_string(name).unwrap().to_str().unwrap().to_owned();

    runtime::spawn(async move {
        let res = note::create_folder(runtime::client(), runtime::config(), name).await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1noteSummaries(_: JNIEnv, _: JObject, wait_id: jint, folder_id: jint) {
    runtime::spawn(async move {
        let res = note::note_summaries(runtime::client(), runtime::config(), folder_id).await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1note(_: JNIEnv, _: JObject, wait_id: jint, note_id: jint) {
    runtime::spawn(async move {
        let res = note::note(runtime::client(), runtime::config(), note_id).await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1createNote(_: JNIEnv, _: JObject, wait_id: jint, folder_id: jint) {
    runtime::spawn(async move {
        let res = note::create_note(runtime::client(), runtime::config(), folder_id).await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1updateNote(env: JNIEnv, _: JObject, wait_id: jint, note_id: jint, text: JString) {
    let text = env.get_string(text).unwrap().to_str().unwrap().to_owned();

    runtime::spawn(async move {
        let res = note::update_note(runtime::client(), runtime::config(), note_id, text).await;

        send(wait_id, res);
    });
}
