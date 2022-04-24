#![allow(non_snake_case)]

use jni::{JNIEnv, objects::{JObject, JString}};

mod log;

#[no_mangle]
pub extern fn Java_com_bwqr_mavinote_viewmodels_Runtime__1init(env: JNIEnv, _: JObject, app_name: JString, api_url: JString, storage_dir: JString) {
    let app_name = env.get_string(app_name).unwrap().to_str().unwrap().to_owned();
    let api_url = env.get_string(api_url).unwrap().to_str().unwrap().to_owned();
    let storage_dir = env.get_string(storage_dir).unwrap().to_str().unwrap().to_owned();

    log::init(app_name);


    ::log::info!("reax runtime is initialized");
}
