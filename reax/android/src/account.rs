use base::State;
use jni::{
    objects::{JString, JClass},
    sys::{jint, jlong},
    JNIEnv
};
use note::Error;
use serde::Serialize;

use crate::{spawn, Message};

fn send_once<T: Serialize>(once_id: i32, message: Result<T, Error>) {
    crate::send_once(once_id, message)
}

fn send_stream<T: Serialize>(stream_id: i32, message: Message<T, Error>) {
    crate::send_stream(stream_id, message)
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1accounts(
    _: JNIEnv,
    _: JClass,
    stream_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let mut rx = note::storage::accounts().await;

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

        send_stream::<()>(stream_id, Message::Complete);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1account(
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
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1devices(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::devices(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1requestVerification(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    email: JString,
) -> jlong {
    let email = env.get_string(email).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::storage::request_verification(email).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1waitVerification(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    token: JString,
) -> jlong {
    let token = env.get_string(token).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::storage::wait_verification(token).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1addAccount(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    email: JString,
) -> jlong {
    let email = env.get_string(email).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::storage::add_account(email).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1publicKey(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::public_key().await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1sendVerificationCode(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    email: JString,
) -> jlong {
    let email = env.get_string(email).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::storage::send_verification_code(email).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1signUp(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    email: JString,
    code: JString,
) -> jlong {
    let email = env.get_string(email).unwrap().to_str().unwrap().to_owned();
    let code = env.get_string(code).unwrap().to_str().unwrap().to_owned();

    let handle = spawn(async move {
        let res = note::storage::sign_up(email, code).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1removeAccount(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
) -> jlong {
    let handle = spawn(async move {
        let res = note::storage::remove_account(account_id).await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}
