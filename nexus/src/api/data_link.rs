use std::ffi::{c_char, c_void};

/// Nexus version of Mumble link data.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct NexusLinkData {
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

pub type RawDataGetResource =
    unsafe extern "C-unwind" fn(identifier: *const c_char) -> *const c_void;

pub type RawDataShareResource =
    unsafe extern "C-unwind" fn(identifier: *const c_char, resource_size: usize) -> *mut c_void;
