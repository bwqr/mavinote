use std::sync::Arc;

use base::{Config, Store, State, Error};
use jni::{
    objects::{JObject, JString},
    sys::{jint, jlong},
    JNIEnv,
};
use reqwest::Client;

use crate::{send_stream, send_once, spawn, Message};

pub fn init() {
    note::init();
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1folders(
    _: JNIEnv,
    _: JObject,
    stream_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let mut rx = note::folders().await;

        match &*rx.borrow() {
            State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
            State::Err(e) => send_stream::<Error>(stream_id, Message::Err(e.clone())),
            _ => {},
        };

        while rx.changed().await.is_ok() {
            match &*rx.borrow() {
                State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
                State::Err(e) => send_stream::<Error>(stream_id, Message::Err(e.clone())),
                _ => {},
            };
        }

        send_stream::<Vec<note::models::Folder>>(stream_id, Message::Complete);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1addFolder(
    env: JNIEnv,
    _: JObject,
    once_id: jint,
    name: JString,
) -> jlong {
    let name = env.get_string(name).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::create_folder(
            runtime::get::<Arc<dyn Store>>().unwrap(),
            runtime::get::<Arc<Client>>().unwrap(),
            runtime::get::<Arc<Config>>().unwrap(),
            name,
        )
        .await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1noteSummaries(
    _: JNIEnv,
    _: JObject,
    stream_id: jint,
    folder_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let mut rx = note::notes(folder_id);

        match &*rx.borrow() {
            State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
            State::Err(e) => send_stream::<Error>(stream_id, Message::Err(e.clone())),
            _ => {},
        };

        while rx.changed().await.is_ok() {
            if let State::Ok(ok) = &*rx.borrow() {
                send_stream(stream_id, Message::Ok(ok.clone()));
            }
        }

        send_stream::<Vec<note::models::Note>>(stream_id, Message::Complete);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1note(
    _: JNIEnv,
    _: JObject,
    once_id: jint,
    note_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::note(
            runtime::get::<Arc<dyn Store>>().unwrap(),
            runtime::get::<Arc<Client>>().unwrap(),
            runtime::get::<Arc<Config>>().unwrap(),
            note_id,
        )
        .await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1createNote(
    _: JNIEnv,
    _: JObject,
    once_id: jint,
    folder_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::create_note(
            runtime::get::<Arc<dyn Store>>().unwrap(),
            runtime::get::<Arc<Client>>().unwrap(),
            runtime::get::<Arc<Config>>().unwrap(),
            folder_id,
        )
        .await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1updateNote(
    env: JNIEnv,
    _: JObject,
    once_id: jint,
    note_id: jint,
    text: JString,
) -> jlong {
    let text = env.get_string(text).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::update_note(
            runtime::get::<Arc<dyn Store>>().unwrap(),
            runtime::get::<Arc<Client>>().unwrap(),
            runtime::get::<Arc<Config>>().unwrap(),
            note_id,
            text,
        )
        .await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}
