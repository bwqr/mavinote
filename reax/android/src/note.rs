use base::{State, Error};
use jni::{
    objects::{JString, JClass},
    sys::{jint, jlong, jboolean},
    JNIEnv
};

use crate::{send_stream, send_once, spawn, Message, block_on};

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1init(
    _: JNIEnv,
    _: JClass,
) {
    block_on(::note::storage::init()).unwrap();

    log::info!("reax note is initialized");
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1sync(
    _: JNIEnv,
    _: JClass,
    once_id: jint
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::sync::sync().await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1accounts(
    _: JNIEnv,
    _: JClass,
    stream_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let mut rx = note::storage::accounts().await;

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
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1account(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1mavinoteAccount(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::mavinote_account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1addAccount(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    name: JString,
    email: JString,
    password: JString,
    create_account: jboolean,
) -> jlong {
    let name = env.get_string(name).unwrap().to_str().unwrap().to_owned();
    let email = env.get_string(email).unwrap().to_str().unwrap().to_owned();
    let password = env.get_string(password).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::storage::add_account(name, email, password, create_account > 0).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1deleteAccount(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::delete_account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1authorizeMavinoteAccount(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
    password: JString,
) -> jlong {
    let password = env.get_string(password).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::storage::authorize_mavinote_account(account_id, password).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1folders(
    _: JNIEnv,
    _: JClass,
    stream_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let mut rx = note::storage::folders().await;

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
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1folder(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    folder_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::folder(folder_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1createFolder(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: i32,
    name: JString,
) -> jlong {
    let name = env.get_string(name).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::storage::create_folder(account_id, name).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1deleteFolder(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    folder_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::delete_folder(folder_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1noteSummaries(
    _: JNIEnv,
    _: JClass,
    stream_id: jint,
    folder_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let mut rx = note::storage::notes(folder_id).await;

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
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1note(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    note_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::note(note_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1createNote(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    folder_id: jint,
    text: JString,
) -> jlong {
    let text = env.get_string(text).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::storage::create_note(folder_id, text).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1updateNote(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    note_id: jint,
    text: JString,
) -> jlong {
    let text = env.get_string(text).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::storage::update_note(note_id, text).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1deleteNote(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    note_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::delete_note(note_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}
