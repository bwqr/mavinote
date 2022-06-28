use jni::{JNIEnv, objects::JObject, sys::{jlong, jint}};

use crate::{spawn, send_stream, send_once, Message};

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_Notify__1start(_: JNIEnv, _: JObject, once_id: jint) -> jlong {
    let handle = spawn(async move {
        let res = notify::start().await;

        send_once(once_id, Ok(res));
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_Notify__1stop(_: JNIEnv, _: JObject, once_id: jint) -> jlong {
    let handle = spawn(async move {
        let res = notify::stop().await;

        send_once(once_id, Ok(res));
    });

    Box::into_raw(Box::new(handle)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_Notify__1connected(_: JNIEnv, _: JObject, stream_id: jint) -> jlong {
    let handle = spawn(async move {
        let mut rx = notify::connected();

        send_stream::<bool>(stream_id, Message::Ok(*rx.borrow()));

        while rx.changed().await.is_ok() {
            send_stream::<bool>(stream_id, Message::Ok(*rx.borrow()));
        }

        send_stream::<bool>(stream_id, Message::Complete);
    });

    Box::into_raw(Box::new(handle)) as jlong
}
