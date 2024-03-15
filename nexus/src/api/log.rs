use std::ffi::c_char;

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
