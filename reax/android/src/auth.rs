use jni::{objects::{JString, JObject}, JNIEnv, sys::jint};

use crate::send;

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_AuthViewModel__1login(env: JNIEnv, _: JObject, wait_id: jint, email: JString, password: JString) {
    let email = env.get_string(email).unwrap().to_str().unwrap().to_owned();
    let password = env.get_string(password).unwrap().to_str().unwrap().to_owned();

    runtime::spawn(async move {
        let res = auth::login(runtime::store(), runtime::client(), runtime::config(), email, password).await;

        send(wait_id, res);
    });
}
