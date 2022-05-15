use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
};

use base::{Config, Store};
use reqwest::Client;

use crate::send;

#[no_mangle]
pub extern "C" fn reax_note_folders(wait_id: c_int) {
    crate::spawn(async move {
        let res = note::folders(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
        )
        .await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern "C" fn reax_note_create_folder(wait_id: c_int, name: *const c_char) {
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() };

    crate::spawn(async move {
        let res = note::create_folder(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            name,
        )
        .await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern "C" fn reax_note_note_summaries(wait_id: c_int, folder_id: c_int) {
    crate::spawn(async move {
        let res = note::note_summaries(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            folder_id,
        )
        .await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern "C" fn reax_note_note(wait_id: c_int, note_id: c_int) {
    crate::spawn(async move {
        let res = note::note(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            note_id,
        )
        .await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern "C" fn reax_note_create_note(wait_id: c_int, folder_id: c_int) {
    crate::spawn(async move {
        let res = note::create_note(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            folder_id,
        )
        .await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern "C" fn reax_note_update_note(wait_id: c_int, note_id: c_int, text: *const c_char) {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap().to_string() };

    crate::spawn(async move {
        let res = note::update_note(
            runtime::get::<Store>().unwrap(),
            runtime::get::<Client>().unwrap(),
            runtime::get::<Config>().unwrap(),
            note_id,
            text,
        )
        .await;

        send(wait_id, res);
    });
}
