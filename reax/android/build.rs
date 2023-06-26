use std::env;
use std::path::PathBuf;

// Due to missing some intrinsic functions in rust and requirement of these functions by sqlite,
// one additional library must be linked.
// This problem become more apparent when NDK 23 drops libgcc
// For more information https://github.com/mozilla/application-services/issues/5436
fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target_arch == "x86_64" && target_os == "android" {
        let mut x86_64_linux_android_libs = PathBuf::from(env::var("CLANG_PATH").expect("CLANG_PATH is not set"));
        x86_64_linux_android_libs.pop();
        x86_64_linux_android_libs.pop();
        x86_64_linux_android_libs.push("lib64");
        x86_64_linux_android_libs.push("clang");
        x86_64_linux_android_libs.push("14.0.7");
        x86_64_linux_android_libs.push("lib");
        x86_64_linux_android_libs.push("linux");

        println!("cargo:rustc-link-search={}", x86_64_linux_android_libs.to_str().unwrap());
        println!("cargo:rustc-link-lib=static=clang_rt.builtins-x86_64-android");
    }
}
