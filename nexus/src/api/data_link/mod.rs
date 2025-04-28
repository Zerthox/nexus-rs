//! Data link for sharing resources.
//!
//! Enable the `"mumble"` or `"mumble_json"` feature for Mumble link bindings.

mod nexus;

/// Mumble link bindings.
#[cfg(feature = "mumble")]
pub mod mumble;

/// RealTime API link bindings.
#[cfg(feature = "rtapi")]
pub mod rtapi;

pub use self::nexus::*;

#[cfg(feature = "mumble")]
pub use self::mumble::{get_mumble_link, get_mumble_link_ptr, read_mumble_link, MumbleLink};

use crate::{util::str_to_c, AddonApi, DataLinkApi};
use std::{
    ffi::{c_char, c_void},
    mem,
};

pub type RawDataGetResource =
    unsafe extern "C-unwind" fn(identifier: *const c_char) -> *const c_void;

pub type RawDataShareResource =
    unsafe extern "C-unwind" fn(identifier: *const c_char, resource_size: usize) -> *mut c_void;

/// Returns a pointer to a shared resource.
pub fn get_resource<T>(identifier: impl AsRef<str>) -> *const T {
    let identifier = str_to_c(identifier, "failed to convert data link identifier");
    let DataLinkApi { get, .. } = AddonApi::get().data_link;
    unsafe { get(identifier.as_ptr()).cast() }
}

/// Reads a shared resource.
///
/// # Safety
/// The caller must ensure the data associated with the given identifier is of type `T`.
pub unsafe fn read_resource<T>(identifier: impl AsRef<str>) -> Option<T> {
    let ptr = get_resource::<T>(identifier);
    let valid = !ptr.is_null();
    valid.then(|| unsafe { ptr.read_volatile() })
}

/// Creates a new shared resource.
pub fn share_resource<T>(identifier: impl AsRef<str>) -> *mut T {
    let identifier = str_to_c(identifier, "failed to convert data link identifier");
    let DataLinkApi { share, .. } = AddonApi::get().data_link;
    let size = mem::size_of::<T>();
    unsafe { share(identifier.as_ptr(), size).cast() }
}
