use std::sync::Arc;

use base::{Store, Config};
use jni::{
    objects::{JObject, JString},
    sys::{jint, jlong},
    JNIEnv,
};
use reqwest::Client;

use crate::{send_once, spawn};

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AuthViewModel__1login(
    env: JNIEnv,
    _: JObject,
    once_id: jint,
    email: JString,
    password: JString,
) -> jlong {
    let email = env.get_string(email).unwrap().to_str().unwrap().to_owned();
    let password = env
        .get_string(password)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let handle = spawn(async move {
        let res = auth::login(
            runtime::get::<Arc<dyn Store>>().unwrap(),
            runtime::get::<Arc<Client>>().unwrap(),
            runtime::get::<Arc<Config>>().unwrap(),
            email,
            password,
        )
        .await;

        send_once(once_id, res);
    });

    Box::into_raw(Box::new(handle)) as jlong
}
