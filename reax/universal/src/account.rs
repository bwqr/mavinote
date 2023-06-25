use base::State;
use note::Error;
use serde::Serialize;
use tokio::task::JoinHandle;

use crate::{spawn, Message};

fn send_once<T: Serialize>(once_id: i32, message: Result<T, Error>) {
    crate::send_once(once_id, message)
}

fn send_stream<T: Serialize>(stream_id: i32, message: Message<T, Error>) {
    crate::send_stream(stream_id, message)
}

pub fn accounts(stream_id: i32) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let mut rx = note::storage::accounts().await;

        match &*rx.borrow() {
            State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
            State::Err(e) => send_stream::<()>(stream_id, Message::Err(e.clone())),
            _ => {}
        };

        while rx.changed().await.is_ok() {
            match &*rx.borrow() {
                State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
                State::Err(e) => send_stream::<()>(stream_id, Message::Err(e.clone())),
                _ => {}
            };
        }

        send_stream::<()>(stream_id, Message::Complete);
    });

    Box::into_raw(Box::new(handle))
}

pub fn account(once_id: i32, account_id: i32) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn mavinote_account(once_id: i32, account_id: i32) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::mavinote_account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn devices(once_id: i32, account_id: i32) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::devices(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn add_device(once_id: i32, account_id: i32, fingerprint: String) -> * mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::add_device(account_id, fingerprint).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn remove_device(once_id: i32, device_id: i32) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::remove_device(device_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn request_verification(once_id: i32, email: String) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::request_verification(email).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn wait_verification(once_id: i32, token: String) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::wait_verification(token).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn add_account(once_id: i32, email: String) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::add_account(email).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn public_key(once_id: i32) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::public_key().await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn send_verification_code(once_id: i32, email: String) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::send_verification_code(email).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn sign_up(once_id: i32, email: String, code: String) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::sign_up(email, code).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn remove_account(once_id: i32, account_id: i32) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::remove_account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn send_account_close_code(once_id: i32, account_id: i32) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::send_account_close_code(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}

pub fn close_account(once_id: i32, account_id: i32, code: String) -> *mut JoinHandle<()> {
    let handle = spawn(async move {
        let res = note::storage::close_account(account_id, code).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle))
}
