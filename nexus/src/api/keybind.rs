use crate::{addon_api, util::str_to_c, AddonApi};
use std::ffi::c_char;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Keybind {
    pub key: u16,
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
}

pub type RawKeybindHandler = extern "C-unwind" fn(identifier: *const c_char);

pub type RawKeybindRegisterWithString = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    keybind_handler: RawKeybindHandler,
    keybind: *const c_char,
);

pub type RawKeybindRegisterWithStruct = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    keybind_handler: RawKeybindHandler,
    keybind: Keybind,
);

pub type RawKeybindDeregister = unsafe extern "C-unwind" fn(identifier: *const c_char);

// TODO: wrapped callbacks

/// Registers a new keybind using a keybind string like `"ALT+SHIFT+T"`.
///
/// Returns a callable that reverts the register.
pub fn register_keybind_with_string_raw(
    identifier: impl AsRef<str>,
    handler: RawKeybindHandler,
    keybind: impl AsRef<str>,
) -> impl Fn() + Send + Sync + Clone + 'static {
    let AddonApi {
        keybind_register_with_string,
        keybind_deregister,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert keybind identifier");
    let keybind = str_to_c(keybind, "failed to convert keybind string");
    unsafe { keybind_register_with_string(identifier.as_ptr(), handler, keybind.as_ptr()) };
    move || unsafe { keybind_deregister(identifier.as_ptr()) }
}

/// Registers a new keybind using a [`Keybind`] struct.
///
/// Returns a callable that reverts the register.
pub fn register_keybind_with_struct_raw(
    identifier: impl AsRef<str>,
    handler: RawKeybindHandler,
    keybind: Keybind,
) -> impl Fn() + Send + Sync + Clone + 'static {
    let AddonApi {
        keybind_register_with_struct,
        keybind_deregister,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert keybind identifier");
    unsafe { keybind_register_with_struct(identifier.as_ptr(), handler, keybind) };
    move || unsafe { keybind_deregister(identifier.as_ptr()) }
}

/// Unregisters a previously registered keybind.
pub fn unregister_keybind(identifier: impl AsRef<str>) {
    let AddonApi {
        keybind_deregister, ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert keybind identifier");
    unsafe { keybind_deregister(identifier.as_ptr()) }
}
