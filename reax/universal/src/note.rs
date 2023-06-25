use base::State;
use note::Error;
use serde::Serialize;
use tokio::task::JoinHandle;

use crate::{spawn, block_on, Message};

fn send_once<T: Serialize>(once_id: i32, message: Result<T, Error>) {
    crate::send_once(once_id, message)
}

fn send_stream<T: Serialize>(stream_id: i32, message: Message<T, Error>) {
    crate::send_stream(stream_id, message)
}

pub fn init() {
    block_on(::note::storage::init()).unwrap();

    log::info!("reax note is initialized");
}

pub fn sync(once_id: i32) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::sync::sync().await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn folders(stream_id: i32) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let mut rx = note::storage::folders().await;

        match &*rx.borrow() {
            State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
            State::Err(e) => send_stream::<()>(stream_id, Message::Err(e.clone())),
            _ => {},
        };

        while rx.changed().await.is_ok() {
            match &*rx.borrow() {
                State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
                State::Err(e) => send_stream::<()>(stream_id, Message::Err(e.clone())),
                _ => {},
            };
        }

        send_stream::<Vec<note::models::Folder>>(stream_id, Message::Complete);
    });

    Box::into_raw(Box::new(handle))
}

pub fn folder(once_id: i32, folder_id: i32) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::folder(folder_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn create_folder(once_id: i32, account_id: i32, name: String) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::create_folder(account_id, name).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn delete_folder(once_id: i32, folder_id: i32) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::delete_folder(folder_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn note_summaries(stream_id: i32, folder_id: i32) -> * mut JoinHandle<()> {
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

pub fn note(once_id: i32, note_id: i32) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::note(note_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn create_note(once_id: i32, folder_id: i32, text: String) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::create_note(folder_id, text).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn update_note(once_id: i32, note_id: i32, text: String) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::update_note(note_id, text).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn delete_note(once_id: i32, note_id: i32) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::delete_note(note_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}
