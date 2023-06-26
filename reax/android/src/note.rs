use jni::{
    objects::{JString, JClass},
    sys::{jint, jlong},
    JNIEnv
};

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1init(
    _: JNIEnv,
    _: JClass,
) {
    universal::note::init();
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1sync(
    _: JNIEnv,
    _: JClass,
    once_id: jint
) -> jlong {
    universal::note::sync(once_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1folders(
    _: JNIEnv,
    _: JClass,
    stream_id: jint,
) -> jlong {
    universal::note::folders(stream_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1folder(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    folder_id: jint,
) -> jlong {
    universal::note::folder(once_id, folder_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1createFolder(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
    name: JString,
) -> jlong {
    let name = env.get_string(name).unwrap().to_str().unwrap().to_owned();

    universal::note::create_folder(once_id, account_id, name) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1deleteFolder(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    folder_id: jint,
) -> jlong {
    universal::note::delete_folder(once_id, folder_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1noteSummaries(
    _: JNIEnv,
    _: JClass,
    stream_id: jint,
    folder_id: jint,
) -> jlong {
    universal::note::note_summaries(stream_id, folder_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1note(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    note_id: jint,
) -> jlong {
    universal::note::note(once_id, note_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1createNote(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    folder_id: jint,
    text: JString,
) -> jlong {
    let text = env.get_string(text).unwrap().to_str().unwrap().to_owned();

    universal::note::create_note(once_id, folder_id, text) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1updateNote(
    env: JNIEnv,
    _: JClass,
    once_id: jint,
    note_id: jint,
    text: JString,
) -> jlong {
    let text = env.get_string(text).unwrap().to_str().unwrap().to_owned();

    universal::note::update_note(once_id, note_id, text) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_NoteViewModelKt__1deleteNote(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    note_id: jint,
) -> jlong {
    universal::note::delete_note(once_id, note_id) as jlong
}
