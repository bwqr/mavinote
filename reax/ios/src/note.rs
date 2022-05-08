use std::{os::raw::{c_int, c_char}, ffi::CStr};

use crate::send;

#[no_mangle]
pub extern fn reax_note_folders(wait_id: c_int) {
    runtime::spawn(async move {
        let res = note::folders(runtime::store(), runtime::client(), runtime::config()).await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern fn reax_note_create_folder(wait_id: c_int, name: * const c_char) {
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() };

    runtime::spawn(async move {
        let res = note::create_folder(runtime::store(), runtime::client(), runtime::config(), name).await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern fn reax_note_note_summaries(wait_id: c_int, folder_id: c_int) {
    runtime::spawn(async move {
        let res = note::note_summaries(runtime::store(), runtime::client(), runtime::config(), folder_id).await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern fn reax_note_note(wait_id: c_int, note_id: c_int) {
    runtime::spawn(async move {
        let res = note::note(runtime::store(), runtime::client(), runtime::config(), note_id).await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern fn reax_note_create_note(wait_id: c_int, folder_id: c_int) {
    runtime::spawn(async move {
        let res = note::create_note(runtime::store(), runtime::client(), runtime::config(), folder_id).await;

        send(wait_id, res);
    });
}

#[no_mangle]
pub extern fn reax_note_update_note(wait_id: c_int, note_id: c_int, text: * const c_char) {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap().to_string() };

    runtime::spawn(async move {
        let res = note::update_note(runtime::store(), runtime::client(), runtime::config(), note_id, text).await;

        send(wait_id, res);
    });
}
