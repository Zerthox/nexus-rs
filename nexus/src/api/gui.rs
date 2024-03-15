use std::ffi::c_void;

pub const IMGUI_VERSION: u32 = 18000; // is this still correct?

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum RenderType {
    PreRender,
    Render,
    PostRender,
    OptionsRender,
}

pub type RawGuiRender = unsafe extern "C-unwind" fn();

pub type RawGuiAddRender =
    unsafe extern "C-unwind" fn(render_type: RenderType, render_callback: RawGuiRender);

pub type RawGuiRemRender = unsafe extern "C-unwind" fn(render_callback: RawGuiRender);

pub type ImguiMalloc = unsafe extern "C" fn(size: usize, user_data: *mut c_void) -> *mut c_void;

pub type ImguiFree = unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void);
