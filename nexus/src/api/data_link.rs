use crate::{addon_api, util::str_to_c, AddonApi};
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
#[repr(C)]
pub struct NexusLink {
    pub width: u32,
    pub height: u32,
    pub scaling: f32,
    pub is_moving: bool,
    pub is_camera_moving: bool,
    pub is_gameplay: bool,
    pub font: *const c_void,
    pub font_big: *const c_void,
    pub font_ui: *const c_void,
}

/// Gets a pointer to a shared resource.
pub fn get_resource<T>(identifier: impl AsRef<str>) -> *const T {
    let identifier = str_to_c(identifier, "failed to convert data link identifier");
    let AddonApi { get_resource, .. } = addon_api();
    unsafe { get_resource(identifier.as_ptr()) as _ }
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
    let AddonApi { share_resource, .. } = addon_api();
    let size = mem::size_of::<T>();
    unsafe { share_resource(identifier.as_ptr(), size) as _ }
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

    pub use gw2_mumble::{
        map_type, Context, Identity, LinkedMem as MumbleLink, Mount, Position, Profession, Race,
        UIScaling, UiState,
    };

    /// Returns the shared [`MumbleLink`] pointer.
    #[inline]
    pub fn get_mumble_link() -> *const MumbleLink {
        get_resource(MUMBLE_LINK)
    }

    /// Reads the shared [`MumbleLink`] data.
    #[inline]
    pub fn read_mumble_link() -> Option<MumbleLink> {
        unsafe { read_resource(MUMBLE_LINK) }
    }
}

#[cfg(feature = "mumble")]
pub use self::mumble::{get_mumble_link, read_mumble_link, MumbleLink};
