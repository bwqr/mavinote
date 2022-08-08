use std::{os::raw::{c_int, c_char}, ffi::CString};

use log::Log;

#[allow(dead_code)]
#[repr(C)]
enum LogPriority {
    Unknown = 0,
    Default,
    Verbose,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Silent,
}

struct Logger;

impl Logger {
    fn init(self) {
        log::set_logger(Box::leak(Box::new(self))).unwrap();
        log::set_max_level(log::LevelFilter::Debug);
    }
}

impl Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let level: i32 = match record.level() {
            log::Level::Trace => LogPriority::Verbose,
            log::Level::Debug => LogPriority::Debug,
            log::Level::Info => LogPriority::Info,
            log::Level::Warn => LogPriority::Warn,
            log::Level::Error => LogPriority::Error,
        } as i32;


        let tag = CString::new(record.metadata().target()).unwrap();

        let c_str = if record.metadata().target().starts_with("sqlx") {
            CString::new(record.args().to_string().replace("\n", "")).unwrap()
        } else {
            CString::new(record.args().to_string()).unwrap()
        };

        unsafe {
            __android_log_write(level, tag.as_ptr() as *const c_char,c_str.as_ptr());
        }
    }

    fn flush(&self) {
    }
}

pub fn init() {
    Logger.init();
}

#[link(name = "log")]
extern "C" {
    pub fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}
