//! Keybind creation.

use crate::{addon_api, revertible::Revertible, util::str_to_c, AddonApi};
use std::ffi::c_char;

/// A keybind.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Keybind {
    /// The key.
    pub key: u16,

    /// Alt modifier.
    pub alt: bool,

    /// Control modifier.
    pub ctrl: bool,

    /// Shift modifier.
    pub shift: bool,
}

impl Keybind {
    /// Creates a new keybind without modifiers.
    #[inline]
    pub fn without_modifiers(key: u16) -> Self {
        Self {
            key,
            alt: false,
            ctrl: false,
            shift: false,
        }
    }

    /// Checks whether the keybind has modifiers.
    #[inline]
    pub fn has_modifiers(&self) -> bool {
        !self.alt && !self.ctrl && !self.shift
    }
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

/// Registers a new keybind using a keybind string like `"ALT+SHIFT+T"`.
///
/// Returns a [`Revertible`] to revert the register.
///
/// # Usage
/// ```no_run
/// use nexus::keybind::{register_keybind_with_string, keybind_handler};
/// let keybind_handler = keybind_handler!(|id| {
///     use nexus::log::{log, LogLevel};
///     log(LogLevel::Info, "My Addon", format!("keybind {id} pressed"));
/// });
/// register_keybind_with_string("MY_KEYBIND", keybind_handler, "ALT+SHIFT+X")
///     .revert_on_unload();
/// ```
pub fn register_keybind_with_string(
    identifier: impl AsRef<str>,
    handler: RawKeybindHandler,
    keybind: impl AsRef<str>,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let AddonApi {
        keybind_register_with_string,
        keybind_deregister,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert keybind identifier");
    let keybind = str_to_c(keybind, "failed to convert keybind string");
    unsafe { keybind_register_with_string(identifier.as_ptr(), handler, keybind.as_ptr()) };
    let revert = move || unsafe { keybind_deregister(identifier.as_ptr()) };
    revert.into()
}

/// Registers a new keybind using a [`Keybind`] struct.
///
/// Returns a [`Revertible`] to revert the register.
///
/// # Usage
/// ```no_run
/// use nexus::keybind::{register_keybind_with_struct, Keybind, keybind_handler};
/// let keybind = Keybind {
///     key: 123,
///     alt: true,
///     ctrl: false,
///     shift: true,
/// };
/// let keybind_handler = keybind_handler!(|id| {
///     use nexus::log::{log, LogLevel};
///     log(LogLevel::Info, "My Addon", format!("keybind {id} pressed"));
/// });
/// register_keybind_with_struct("MY_KEYBIND", keybind_handler, keybind)
///     .revert_on_unload();
/// ```
pub fn register_keybind_with_struct(
    identifier: impl AsRef<str>,
    handler: RawKeybindHandler,
    keybind: Keybind,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let AddonApi {
        keybind_register_with_struct,
        keybind_deregister,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert keybind identifier");
    unsafe { keybind_register_with_struct(identifier.as_ptr(), handler, keybind) };
    let revert = move || unsafe { keybind_deregister(identifier.as_ptr()) };
    revert.into()
}

/// Unregisters a previously registered keybind.
pub fn unregister_keybind(identifier: impl AsRef<str>) {
    let AddonApi {
        keybind_deregister, ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert keybind identifier");
    unsafe { keybind_deregister(identifier.as_ptr()) }
}

/// Macro to wrap a keybind handler callback.
///
/// Generates a [`RawKeybindHandler`] wrapper around the passed callback.
///
/// # Usage
/// ```no_run
/// # use nexus::keybind::*;
/// let keybind_handler: RawKeybindHandler = keybind_handler!(|id| {
///     use nexus::log::{log, LogLevel};
///     log(LogLevel::Info, "My Addon", format!("keybind {id} pressed"));
/// });
/// ```
#[macro_export]
macro_rules! keybind_handler {
    ( $callback:expr $(,)? ) => {{
        const __CALLBACK: fn(&::std::primitive::str) = $callback;

        extern "C-unwind" fn __keybind_callback_wrapper(identifier: *const ::std::ffi::c_char) {
            let identifier = unsafe { $crate::__macro::str_from_c(identifier) }
                .expect("invalid identifier in keybind callback");
            __CALLBACK(identifier)
        }

        __keybind_callback_wrapper
    }};
}

pub use keybind_handler;
