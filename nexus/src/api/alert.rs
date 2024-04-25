//! Alert notifications displayed to the user.

use crate::{addon_api, util::str_to_c, AddonApi};
use std::ffi::c_char;

pub type RawAlertNotify = unsafe extern "C-unwind" fn(message: *const c_char);

/// Sends an alert that is visible to the user for a short amount of time.
pub fn alert_notify(message: impl AsRef<str>) {
    let message = str_to_c(message, "failed to convert alert message");
    let AddonApi { alert_notify, .. } = addon_api();
    unsafe { alert_notify(message.as_ptr()) }
}
