use super::{get_resource, read_resource};
use imgui::sys::ImFont;

/// Nexus link identifier.
pub const NEXUS_LINK: &str = NexusLink::LINK;

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

impl NexusLink {
    /// Nexus link identifier.
    pub const LINK: &str = "DL_NEXUS_LINK";

    /// Returns the shared [`NexusLink`] pointer.
    #[inline]
    pub fn get() -> *const Self {
        get_resource(Self::LINK)
    }

    /// Reads the shared [`NexusLink`] data.
    #[inline]
    pub fn read() -> Option<NexusLink> {
        unsafe { read_resource(Self::LINK) }
    }
}

/// Returns the shared [`NexusLink`] pointer.
#[inline]
pub fn get_nexus_link() -> *const NexusLink {
    NexusLink::get()
}

/// Reads the shared [`NexusLink`] data.
#[inline]
pub fn read_nexus_link() -> Option<NexusLink> {
    NexusLink::read()
}
