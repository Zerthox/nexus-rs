use crate::{addon_api, AddonApi};
use std::ffi::{c_char, CString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum LogLevel {
    Off = 0,
    Critical = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
    All,
}

/// Previous log function.
pub type RawLogOld = unsafe extern "C-unwind" fn(level: LogLevel, message: *const c_char);

pub type RawLog =
    unsafe extern "C-unwind" fn(level: LogLevel, channel: *const c_char, message: *const c_char);

/// Logs a message to the given channel.
///
/// Supports custom coloring for addon window messages, for example `<c=#FF0000>this text is red</c>`.
pub fn log(level: LogLevel, channel_name: impl AsRef<str>, message: impl AsRef<str>) {
    let AddonApi { log, .. } = addon_api();
    let channel = CString::new(channel_name.as_ref()).expect("failed to convert log channel");
    let message = CString::new(message.as_ref()).expect("failed to convert log message");
    unsafe { log(level, channel.as_ptr(), message.as_ptr()) }
}
