use base::{State, Error};
use jni::{
    objects::{JObject, JString},
    sys::{jint, jlong},
    JNIEnv,
};

use crate::{send_stream, send_once, spawn, Message};

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1sync(
    _: JNIEnv,
    _: JObject,
    once_id: jint
) -> jlong {
    let handle = spawn(async move {
        let res = note::sync().await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1activeSyncs(
    _: JNIEnv,
    _: JObject,
    stream_id: jint
) -> jlong {
    let handle = spawn(async move {
        let mut rx = note::active_syncs();

        send_stream::<i32>(stream_id, Message::Ok(*rx.borrow()));

        while rx.changed().await.is_ok() {
            send_stream::<i32>(stream_id, Message::Ok(*rx.borrow()));
        }

        send_stream::<i32>(stream_id, Message::Complete);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1accounts(
    _: JNIEnv,
    _: JObject,
    stream_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let mut rx = note::accounts().await;

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

        send_stream::<Vec<note::models::Account>>(stream_id, Message::Complete);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1addAccount(
    env: JNIEnv,
    _: JObject,
    once_id: jint,
    email: JString,
    password: JString,
) -> jlong {
    let email = env.get_string(email).unwrap().to_str().unwrap().to_owned();
    let password = env.get_string(password).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::create_account(email, password).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1deleteAccount(
    _: JNIEnv,
    _: JObject,
    once_id: jint,
    account_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::delete_account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
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
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1folder(
    _: JNIEnv,
    _: JObject,
    once_id: jint,
    folder_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::folder(folder_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1addFolder(
    env: JNIEnv,
    _: JObject,
    once_id: jint,
    account_id: i32,
    name: JString,
) -> jlong {
    let name = env.get_string(name).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::create_folder(account_id, name).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1deleteFolder(
    _: JNIEnv,
    _: JObject,
    once_id: jint,
    folder_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::delete_folder(folder_id).await;

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
        let mut rx = note::notes(folder_id).await;

        match &*rx.inner().borrow() {
            State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
            State::Err(e) => send_stream::<Error>(stream_id, Message::Err(e.clone())),
            _ => {},
        };

        while rx.inner().changed().await.is_ok() {
            if let State::Ok(ok) = &*rx.inner().borrow() {
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
        let res = note::note(note_id).await;

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
        let res = note::create_note(folder_id).await;

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
        let res = note::update_note(note_id, text).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModel__1deleteNote(
    _: JNIEnv,
    _: JObject,
    once_id: jint,
    note_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::delete_note(note_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}
