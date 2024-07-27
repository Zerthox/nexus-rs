//! Alert notifications displayed to the user.

use crate::{util::str_to_c, AddonApi, UiApi};
use std::ffi::c_char;

pub type RawAlertNotify = unsafe extern "C-unwind" fn(message: *const c_char);

/// Sends an alert that is visible to the user for a short amount of time.
pub fn send_alert(message: impl AsRef<str>) {
    let message = str_to_c(message, "failed to convert alert message");
    let UiApi { send_alert, .. } = AddonApi::get().ui;
    unsafe { send_alert(message.as_ptr()) }
}
