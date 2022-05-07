use std::os::raw::c_int;

use crate::send;

#[no_mangle]
pub extern fn reax_note_folders(wait_id: c_int) {
    runtime::spawn(async move {
        let res = note::folders(runtime::store(), runtime::client(), runtime::config()).await;

        send(wait_id, res);
    });
}

