use std::ffi::{CStr, c_char};

use tokio::task::JoinHandle;

#[no_mangle]
pub extern "C" fn reax_note_init() {
    universal::note::init();
}

#[no_mangle]
pub extern "C" fn reax_note_create_folder(once_id: i32, account_id: i32, name: * const c_char) -> * mut JoinHandle<()> {
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() };

    universal::note::create_folder(once_id, account_id, name)
}

#[no_mangle]
pub extern "C" fn reax_note_create_note(once_id: i32, folder_id: i32, text: * const c_char) -> * mut JoinHandle<()> {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap().to_string() };

    universal::note::create_note(once_id, folder_id, text)
}

#[no_mangle]
pub extern "C" fn reax_note_delete_folder(once_id: i32, folder_id: i32) -> * mut JoinHandle<()> {
    universal::note::delete_folder(once_id, folder_id)
}

#[no_mangle]
pub extern "C" fn reax_note_delete_note(once_id: i32, note_id: i32) -> * mut JoinHandle<()> {
    universal::note::delete_note(once_id, note_id)
}

#[no_mangle]
pub extern "C" fn reax_note_folder(once_id: i32, folder_id: i32) -> * mut JoinHandle<()> {
    universal::note::folder(once_id, folder_id)
}

#[no_mangle]
pub extern "C" fn reax_note_folders(stream_id: i32) -> * mut JoinHandle<()> {
    universal::note::folders(stream_id)
}

#[no_mangle]
pub extern "C" fn reax_note_note(once_id: i32, note_id: i32) -> * mut JoinHandle<()> {
    universal::note::note(once_id, note_id)
}

#[no_mangle]
pub extern "C" fn reax_note_note_summaries(stream_id: i32, folder_id: i32) -> * mut JoinHandle<()> {
    universal::note::note_summaries(stream_id, folder_id)
}

#[no_mangle]
pub extern "C" fn reax_note_update_note(once_id: i32, note_id: i32, text: * const c_char) -> * mut JoinHandle<()> {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap().to_string() };

    universal::note::update_note(once_id, note_id, text)
}

#[no_mangle]
pub extern "C" fn reax_note_sync(once_id: i32) -> * mut JoinHandle<()> {
    universal::note::sync(once_id)
}
