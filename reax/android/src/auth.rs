use base::{Store, Config};
use jni::{
    objects::{JObject, JString},
    sys::jint,
    JNIEnv,
};
use reqwest::Client;

use crate::send;

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AuthViewModel__1login(
    env: JNIEnv,
    _: JObject,
    wait_id: jint,
    email: JString,
    password: JString,
) {
    let email = env.get_string(email).unwrap().to_str().unwrap().to_owned();
    let password = env
        .get_string(password)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    crate::spawn(async move {
        let res = auth::login(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            email,
            password,
        )
        .await;

        send(wait_id, res);
    });
}
