use std::ffi::c_void;

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
