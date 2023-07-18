#![allow(non_snake_case)]

use std::ffi::CString;

use jni::{
    objects::{JObject, JString, JClass, JValue, JByteArray},
    signature::{Primitive, ReturnType},
    JNIEnv, sys::jlong,
};
use libc::c_char;

mod account;
mod log;
mod note;

fn capture_stderr() {
    std::thread::spawn(|| unsafe {
        let mut pipes: [i32; 2] = [0; 2];
        libc::pipe(&mut pipes as *mut i32);
        libc::dup2(pipes[1], libc::STDERR_FILENO);

        let readonly = CString::new("r").unwrap();
        let file = libc::fdopen(pipes[0], readonly.as_ptr());

        let mut buff: [c_char; 256] = [0; 256];
        let tag = CString::new("stderr").unwrap();

        loop {
            libc::fgets(&mut buff as *mut c_char, 256, file);
            log::__android_log_write(5, tag.as_ptr(), buff.as_ptr());
        }
    });
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_reax_RuntimeKt__1init<'a>(
    mut env: JNIEnv<'a>,
    _: JClass,
    api_url: JString,
    ws_url: JString,
    storage_dir: JString,
) -> JByteArray<'a> {
    capture_stderr();

    let api_url = env
        .get_string(&api_url)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let ws_url = env
        .get_string(&ws_url)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let storage_dir = env
        .get_string(&storage_dir)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    log::init();

    bytes_to_byte_array(&env, universal::init(api_url, ws_url, storage_dir.clone()))
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_reax_RuntimeKt__1initHandler(
    mut env: JNIEnv,
    _: JClass,
    callback: JObject,
) {
    let (send, recv) = std::sync::mpsc::channel();
    universal::init_handler(send);

    let callback_class = env.get_object_class(&callback).unwrap();
    let callback_method_id = env
        .get_method_id(callback_class, "invoke", "(I[B)V")
        .unwrap();

    while let Ok((wait_id, bytes)) = recv.recv() {
        let bytes_array = env.new_byte_array(bytes.len().try_into().unwrap()).unwrap();
        env.set_byte_array_region(
            &bytes_array,
            0,
            bytes
                .into_iter()
                .map(|byte| byte as i8)
                .collect::<Vec<i8>>()
                .as_slice(),
        )
        .unwrap();

        let res = unsafe {
            env.call_method_unchecked(
                &callback,
                callback_method_id,
                ReturnType::Primitive(Primitive::Void),
                &[
                    JValue::Int(wait_id).as_jni(),
                    JValue::Object(&bytes_array).as_jni(),
                ],
            )
        };

        if let Err(e) = res {
            ::log::error!("failed to call storeHandler, {:?}", e);
        }

        if let Ok(true) = env.exception_check() {
            ::log::error!("exception is occured");
            env.exception_describe().unwrap();
            env.exception_clear().unwrap();
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_reax_RuntimeKt__1abort(
    _: JNIEnv,
    _: JClass,
    pointer: jlong,
) {
    unsafe { universal::abort(pointer as _); }
}

fn bytes_to_byte_array<'a>(env: &JNIEnv<'a>, bytes: Vec<u8>) -> JByteArray<'a> {
    let bytes_array = env.new_byte_array(bytes.len().try_into().unwrap()).unwrap();

    env.set_byte_array_region(
        &bytes_array,
        0,
        bytes
            .into_iter()
            .map(|byte| byte as i8)
            .collect::<Vec<i8>>()
            .as_slice(),
    )
    .unwrap();

    bytes_array
}
