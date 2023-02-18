use jni::{
    objects::{JString, JClass},
    sys::{jint, jlong},
    JNIEnv
};

use crate::{send_once, spawn};

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
