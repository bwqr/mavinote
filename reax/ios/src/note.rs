use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
};

use base::{State, Error};
use tokio::task::JoinHandle;

use crate::{spawn, send_stream, Message, send_once};

#[no_mangle]
pub extern "C" fn reax_note_sync(once_id: c_int) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::sync::sync().await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_folders(stream_id: c_int) -> * mut JoinHandle<()> {
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

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_folder(once_id: c_int, folder_id: c_int) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::folder(folder_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_create_folder(once_id: c_int, name: *const c_char) -> * mut JoinHandle<()>  {
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() };

    let handle = spawn(async move {
        let res = note::create_folder(1, name).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_delete_folder(once_id: c_int, folder_id: c_int) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::delete_folder(folder_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_note_summaries(stream_id: c_int, folder_id: c_int) -> * mut JoinHandle<()>  {
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

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_note(once_id: c_int, note_id: c_int) -> * mut JoinHandle<()>  {
    let handle = spawn(async move {
        let res = note::note(note_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_create_note(once_id: c_int, folder_id: c_int, text: *const c_char) -> * mut JoinHandle<()>  {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap().to_string() };

    let handle = spawn(async move {
        let res = note::create_note(folder_id, text).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_update_note(once_id: c_int, note_id: c_int, text: *const c_char) -> * mut JoinHandle<()>  {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap().to_string() };

    let handle = spawn(async move {
        let res = note::update_note(note_id, text).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn reax_note_delete_note(once_id: c_int, note_id: c_int) -> * mut JoinHandle<()>  {
    let handle = spawn(async move {
        let res = note::delete_note(note_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}
