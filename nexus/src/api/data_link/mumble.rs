pub use gw2_mumble::{LinkedMem as MumbleLink, *};

use super::{get_resource, read_resource};

/// Mumble link identifier.
pub const MUMBLE_LINK: &str = "DL_MUMBLE_LINK";

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
