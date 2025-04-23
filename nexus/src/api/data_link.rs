//! Data link for sharing resources.
//!
//! Enable the `"mumble"` or `"mumble_json"` feature for Mumble link bindings.

use crate::{util::str_to_c, AddonApi, DataLinkApi};
use imgui::sys::ImFont;
use std::{
    ffi::{c_char, c_void},
    mem,
};

pub type RawDataGetResource =
    unsafe extern "C-unwind" fn(identifier: *const c_char) -> *const c_void;

pub type RawDataShareResource =
    unsafe extern "C-unwind" fn(identifier: *const c_char, resource_size: usize) -> *mut c_void;

/// Mumble link identifier.
pub const MUMBLE_LINK: &str = "DL_MUMBLE_LINK";

/// Nexus link identifier.
pub const NEXUS_LINK: &str = "DL_NEXUS_LINK";

/// Nexus link data.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(C)]
pub struct NexusLink {
    pub width: u32,
    pub height: u32,
    pub scaling: f32,
    pub is_moving: bool,
    pub is_camera_moving: bool,
    pub is_gameplay: bool,

    #[cfg_attr(feature = "serde", serde(skip))]
    pub font: *mut ImFont,

    #[cfg_attr(feature = "serde", serde(skip))]
    pub font_big: *mut ImFont,

    #[cfg_attr(feature = "serde", serde(skip))]
    pub font_ui: *mut ImFont,
}

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

/// Returns the shared [`NexusLink`] pointer.
#[inline]
pub fn get_nexus_link() -> *const NexusLink {
    get_resource(NEXUS_LINK)
}

/// Reads the shared [`NexusLink`] data.
#[inline]
pub fn read_nexus_link() -> Option<NexusLink> {
    unsafe { read_resource(NEXUS_LINK) }
}

/// Mumble link bindings.
#[cfg(feature = "mumble")]
pub mod mumble {
    use super::*;

    pub use gw2_mumble::{LinkedMem as MumbleLink, *};

    /// Returns the shared [`MumbleLink`] pointer.
    #[inline]
    pub fn get_mumble_link_ptr() -> *const MumbleLink {
        get_resource(MUMBLE_LINK)
    }

    /// Returns the shared [`MumbleLink`] pointer as [`MumblePtr`].
    #[inline]
    pub fn get_mumble_link() -> Option<MumblePtr> {
        let ptr = get_mumble_link_ptr().cast_mut();
        unsafe { MumblePtr::new(ptr) }
    }

    /// Reads the shared [`MumbleLink`] data.
    #[inline]
    pub fn read_mumble_link() -> Option<MumbleLink> {
        unsafe { read_resource(MUMBLE_LINK) }
    }
}

#[cfg(feature = "mumble")]
pub use self::mumble::{get_mumble_link, get_mumble_link_ptr, read_mumble_link, MumbleLink};

#[cfg(feature = "rtapi")]
pub mod rtapi {
    #[inline]
    pub fn get_rtapi_ptr() -> *const crate::rtapi::raw::RealTimeData {
        super::get_resource(crate::rtapi::raw::DL_RTAPI)
    }

    #[inline]
    pub fn read_rtapi() -> Option<crate::rtapi::raw::RealTimeData> {
        unsafe { super::read_resource(crate::rtapi::raw::DL_RTAPI) }
    }

    #[inline]
    pub fn read_rtapi_owned() -> Option<crate::rtapi::RealTimeData> {
        read_rtapi().map(|rtd: crate::rtapi::raw::RealTimeData| rtd.into())
    }
}
