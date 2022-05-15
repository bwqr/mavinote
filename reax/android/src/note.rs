use base::{Config, Store};
use jni::{
    objects::{JObject, JString},
    sys::jint,
    JNIEnv,
};
use reqwest::Client;

use crate::send;

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1folders(
    _: JNIEnv,
    _: JObject,
    wait_id: jint,
) {
    crate::spawn(async move {
        let res = note::folders(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
        )
        .await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1addFolder(
    env: JNIEnv,
    _: JObject,
    wait_id: jint,
    name: JString,
) {
    let name = env.get_string(name).unwrap().to_str().unwrap().to_owned();

    crate::spawn(async move {
        let res = note::create_folder(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            name,
        )
        .await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1noteSummaries(
    _: JNIEnv,
    _: JObject,
    wait_id: jint,
    folder_id: jint,
) {
    crate::spawn(async move {
        let res = note::note_summaries(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            folder_id,
        )
        .await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1note(
    _: JNIEnv,
    _: JObject,
    wait_id: jint,
    note_id: jint,
) {
    crate::spawn(async move {
        let res = note::note(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            note_id,
        )
        .await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1createNote(
    _: JNIEnv,
    _: JObject,
    wait_id: jint,
    folder_id: jint,
) {
    crate::spawn(async move {
        let res = note::create_note(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            folder_id,
        )
        .await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1updateNote(
    env: JNIEnv,
    _: JObject,
    wait_id: jint,
    note_id: jint,
    text: JString,
) {
    let text = env.get_string(text).unwrap().to_str().unwrap().to_owned();

    crate::spawn(async move {
        let res = note::update_note(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            note_id,
            text,
        )
        .await;

        send(wait_id, res);
    });
}
