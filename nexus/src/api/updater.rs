//! Update management.

use crate::{AddonApi, ffi::str_to_c};
use std::ffi::c_char;

pub type RawRequestUpdate = unsafe extern "C-unwind" fn(signature: i32, update_url: *const c_char);

/// Requests an update to be downloaded **without** performing a version check.
pub fn request_update(signature: i32, update_url: impl AsRef<str>) {
    let AddonApi { request_update, .. } = AddonApi::get();
    let update_url = str_to_c(update_url).expect("failed to convert update url");
    unsafe { request_update(signature, update_url.as_ptr()) }
}
