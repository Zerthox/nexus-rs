//! Addon keybinds.

use crate::{revertible::Revertible, util::str_to_c, AddonApi, InputBindsApi};
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

pub type RawKeybindHandler = extern "C-unwind" fn(identifier: *const c_char, is_release: bool);

pub type RawKeybindInvoke =
    unsafe extern "C-unwind" fn(identifier: *const c_char, is_release: bool);

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

pub type RawKeybindHandlerOld = extern "C-unwind" fn(identifier: *const c_char);

pub type RawKeybindRegisterWithStringOld = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    keybind_handler: RawKeybindHandlerOld,
    keybind: *const c_char,
);

pub type RawKeybindRegisterWithStructOld = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    keybind_handler: RawKeybindHandlerOld,
    keybind: Keybind,
);

/// Triggers a previously registered keybind programmatically.
pub fn invoke_keybind(identifier: impl AsRef<str>, is_release: bool) {
    let InputBindsApi { invoke, .. } = AddonApi::get().input_binds;
    let identifier = str_to_c(identifier, "failed to convert keybind identifier");
    unsafe { invoke(identifier.as_ptr(), is_release) }
}

/// Registers a new keybind using a keybind string like `"ALT+SHIFT+T"`.
///
/// Returns a [`Revertible`] to revert the register.
///
/// # Usage
/// ```no_run
/// use nexus::keybind::{register_keybind_with_string, keybind_handler};
/// let keybind_handler = keybind_handler!(|id, is_release| {
///     use nexus::log::{log, LogLevel};
///     log(LogLevel::Info, "My Addon", format!(
///         "keybind {id} {}",
///         if is_release { "released" } else { "pressed "},
///     ));
/// });
/// register_keybind_with_string("MY_KEYBIND", keybind_handler, "ALT+SHIFT+X")
///     .revert_on_unload();
/// ```
pub fn register_keybind_with_string(
    identifier: impl AsRef<str>,
    handler: RawKeybindHandler,
    keybind: impl AsRef<str>,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let InputBindsApi {
        register_with_string,
        deregister,
        ..
    } = AddonApi::get().input_binds;
    let identifier = str_to_c(identifier, "failed to convert keybind identifier");
    let keybind = str_to_c(keybind, "failed to convert keybind string");
    unsafe { register_with_string(identifier.as_ptr(), handler, keybind.as_ptr()) };
    let revert = move || unsafe { deregister(identifier.as_ptr()) };
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
/// let keybind_handler = keybind_handler!(|id, is_release| {
///     use nexus::log::{log, LogLevel};
///     log(LogLevel::Info, "My Addon", format!(
///         "keybind {id} {}",
///         if is_release { "released" } else { "pressed "},
///     ));
/// });
/// register_keybind_with_struct("MY_KEYBIND", keybind_handler, keybind)
///     .revert_on_unload();
/// ```
pub fn register_keybind_with_struct(
    identifier: impl AsRef<str>,
    handler: RawKeybindHandler,
    keybind: Keybind,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let InputBindsApi {
        register_with_struct,
        deregister,
        ..
    } = AddonApi::get().input_binds;
    let identifier = str_to_c(identifier, "failed to convert keybind identifier");
    unsafe { register_with_struct(identifier.as_ptr(), handler, keybind) };
    let revert = move || unsafe { deregister(identifier.as_ptr()) };
    revert.into()
}

/// Unregisters a previously registered keybind.
pub fn unregister_keybind(identifier: impl AsRef<str>) {
    let InputBindsApi { deregister, .. } = AddonApi::get().input_binds;
    let identifier = str_to_c(identifier, "failed to convert keybind identifier");
    unsafe { deregister(identifier.as_ptr()) }
}

/// Macro to wrap a keybind handler callback.
///
/// Generates a [`RawKeybindHandler`] wrapper around the passed callback.
///
/// # Usage
/// ```no_run
/// # use nexus::keybind::*;
/// let keybind_handler: RawKeybindHandler = keybind_handler!(|id, is_release| {
///     use nexus::log::{log, LogLevel};
///     log(LogLevel::Info, "My Addon", format!(
///         "keybind {id} {}",
///         if is_release { "released" } else { "pressed "},
///     ));
/// });
/// ```
#[macro_export]
macro_rules! keybind_handler {
    ( $callback:expr $(,)? ) => {{
        const __CALLBACK: fn(&::std::primitive::str, ::std::primitive::bool) = $callback;

        extern "C-unwind" fn __keybind_callback_wrapper(
            identifier: *const ::std::ffi::c_char,
            is_release: ::std::primitive::bool,
        ) {
            let identifier = unsafe { $crate::__macro::str_from_c(identifier) }
                .expect("invalid identifier in keybind callback");
            __CALLBACK(identifier, is_release)
        }

        __keybind_callback_wrapper
    }};
}

pub use keybind_handler;
