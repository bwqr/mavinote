use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
};

use base::State;
use note::Error;
use serde::Serialize;
use tokio::task::JoinHandle;

use crate::{spawn, Message, block_on};

fn send_once<T: Serialize>(once_id: i32, message: Result<T, Error>) {
    crate::send_once(once_id, message)
}

fn send_stream<T: Serialize>(stream_id: i32, message: Message<T, Error>) {
    crate::send_stream(stream_id, message)
}

#[no_mangle]
pub extern "C" fn reax_note_init() {
    block_on(::note::storage::init()).unwrap();

    log::info!("reax note is initialized");
}

#[no_mangle]
pub extern "C" fn reax_note_accounts(stream_id: c_int) -> * mut JoinHandle<()> {
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

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_account(once_id: c_int, account_id: c_int) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_mavinote_account(once_id: c_int, account_id: c_int) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::mavinote_account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_add_account(once_id: c_int, name: * const c_char, email: * const c_char, code: * const c_char) -> * mut JoinHandle<()> {
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() };
    let email = unsafe { CStr::from_ptr(email).to_str().unwrap().to_string() };
    let code = unsafe { CStr::from_ptr(code).to_str().unwrap().to_string() };

    let handle = spawn(async move {
        let res = note::storage::sign_up(name, email, code).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_delete_account(once_id: c_int, account_id: c_int) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::delete_account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_sync(once_id: c_int) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::sync::sync().await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_folders(stream_id: c_int) -> * mut JoinHandle<()> {
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

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_folder(once_id: c_int, folder_id: c_int) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::folder(folder_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_create_folder(once_id: c_int, account_id: c_int, name: *const c_char) -> * mut JoinHandle<()>  {
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() };

    let handle = spawn(async move {
        let res = note::storage::create_folder(account_id, name).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_delete_folder(once_id: c_int, folder_id: c_int) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::delete_folder(folder_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_note_summaries(stream_id: c_int, folder_id: c_int) -> * mut JoinHandle<()>  {
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

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_note(once_id: c_int, note_id: c_int) -> * mut JoinHandle<()>  {
    let handle = spawn(async move {
        let res = note::storage::note(note_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_create_note(once_id: c_int, folder_id: c_int, text: *const c_char) -> * mut JoinHandle<()>  {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap().to_string() };

    let handle = spawn(async move {
        let res = note::storage::create_note(folder_id, text).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_update_note(once_id: c_int, note_id: c_int, text: *const c_char) -> * mut JoinHandle<()>  {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap().to_string() };

    let handle = spawn(async move {
        let res = note::storage::update_note(note_id, text).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_delete_note(once_id: c_int, note_id: c_int) -> * mut JoinHandle<()>  {
    let handle = spawn(async move {
        let res = note::storage::delete_note(note_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}
