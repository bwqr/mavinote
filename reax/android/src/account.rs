use jni::{
    objects::{JClass, JString},
    sys::{jint, jlong, jboolean},
    JNIEnv,
};

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1accounts(
    _: JNIEnv,
    _: JClass,
    stream_id: jint,
) -> jlong {
    universal::account::accounts(stream_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1account(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
) -> jlong {
    universal::account::account(once_id, account_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1mavinoteAccount(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
) -> jlong {
    universal::account::mavinote_account(once_id, account_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1devices(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
) -> jlong {
    universal::account::devices(once_id, account_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1addDevice(
    mut env: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
    fingerprint: JString,
) -> jlong {
    let fingerprint = env.get_string(&fingerprint).unwrap().to_str().unwrap().to_owned();

    universal::account::add_device(once_id, account_id, fingerprint) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1deleteDevice(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
    device_id: jint,
) -> jlong {
    universal::account::delete_device(once_id, account_id, device_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1requestVerification(
    mut env: JNIEnv,
    _: JClass,
    once_id: jint,
    email: JString,
) -> jlong {
    let email = env.get_string(&email).unwrap().to_str().unwrap().to_owned();

    universal::account::request_verification(once_id, email) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1waitVerification(
    mut env: JNIEnv,
    _: JClass,
    once_id: jint,
    token: JString,
) -> jlong {
    let token = env.get_string(&token).unwrap().to_str().unwrap().to_owned();

    universal::account::wait_verification(once_id, token) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1addAccount(
    mut env: JNIEnv,
    _: JClass,
    once_id: jint,
    email: JString,
) -> jlong {
    let email = env.get_string(&email).unwrap().to_str().unwrap().to_owned();

    universal::account::add_account(once_id, email) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1publicKey(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
) -> jlong {
    universal::account::public_key(once_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1sendVerificationCode(
    mut env: JNIEnv,
    _: JClass,
    once_id: jint,
    email: JString,
) -> jlong {
    let email = env.get_string(&email).unwrap().to_str().unwrap().to_owned();

    universal::account::send_verification_code(once_id, email) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1signUp(
    mut env: JNIEnv,
    _: JClass,
    once_id: jint,
    email: JString,
    code: JString,
) -> jlong {
    let email = env.get_string(&email).unwrap().to_str().unwrap().to_owned();
    let code = env.get_string(&code).unwrap().to_str().unwrap().to_owned();

    universal::account::sign_up(once_id, email, code) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1removeAccount(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
) -> jlong {
    universal::account::remove_account(once_id, account_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1sendAccountCloseCode(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
) -> jlong {
    universal::account::send_account_close_code(once_id, account_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1closeAccount(
    mut env: JNIEnv,
    _: JClass,
    once_id: jint,
    account_id: jint,
    code: JString,
) -> jlong {
    let code = env.get_string(&code).unwrap().to_str().unwrap().to_owned();

    universal::account::close_account(once_id, account_id, code) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1listenNotifications(
    _: JNIEnv,
    _: JClass,
    stream_id: jint,
    account_id: jint,
) -> jlong {
    universal::account::listen_notifications(stream_id, account_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1welcomeShown(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
) -> jlong {
    universal::account::welcome_shown(once_id) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_bwqr_mavinote_viewmodels_AccountViewModelKt__1updateWelcomeShown(
    _: JNIEnv,
    _: JClass,
    once_id: jint,
    shown: jboolean,
) -> jlong {
    universal::account::update_welcome_shown(once_id, shown > 0) as jlong
}
