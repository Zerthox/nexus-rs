//! Logging.

use crate::{addon_api, util::str_to_c, AddonApi};
use std::ffi::c_char;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIter,
        strum::IntoStaticStr,
        strum::VariantArray,
        strum::VariantNames
    )
)]
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
#[inline]
pub fn log(level: LogLevel, channel_name: impl AsRef<str>, message: impl AsRef<str>) {
    let AddonApi { log, .. } = addon_api();
    let channel = str_to_c(channel_name, "failed to convert log channel");
    let message = str_to_c(message, "failed to convert log message");
    unsafe { log(level, channel.as_ptr(), message.as_ptr()) }
}
