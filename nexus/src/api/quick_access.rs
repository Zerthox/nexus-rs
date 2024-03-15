use super::gui::RawGuiRender;
use std::ffi::c_char;

pub type RawQuickAccessAddShortcut = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    texture_identifier: *const c_char,
    texture_hover_identifier: *const c_char,
    keybind_identifier: *const c_char,
    tooltip_text: *const c_char,
);

pub type RawQuickAccessAddSimple =
    unsafe extern "C-unwind" fn(identifier: *const c_char, shortcut_render_callback: RawGuiRender);

pub type RawQuickAccessGeneric = unsafe extern "C-unwind" fn(identifier: *const c_char);
