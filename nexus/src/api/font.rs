//! Font loading.

use crate::{
    util::{path_to_c, str_to_c, OptionRefExt},
    AddonApi, FontApi, Revertible,
};
use imgui::sys::{ImFont, ImFontConfig};
use std::{
    ffi::{c_char, c_void},
    path::Path,
};
use windows::Win32::Foundation::HMODULE;

pub type RawFontReceive = unsafe extern "C-unwind" fn(identifier: *const c_char, font: *mut ImFont);

pub type RawFontGet =
    unsafe extern "C-unwind" fn(identifier: *const c_char, callback: RawFontReceive);

pub type RawFontRelease =
    unsafe extern "C-unwind" fn(identifier: *const c_char, callback: RawFontReceive);

pub type RawFontAddFromFile = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    font_size: f32,
    filename: *const c_char,
    callback: RawFontReceive,
    config: *const ImFontConfig,
);

pub type RawFontAddFromResource = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    font_size: f32,
    resource_id: u32,
    module: HMODULE,
    callback: RawFontReceive,
    config: *const ImFontConfig,
);

pub type RawFontAddFromMemory = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    font_size: f32,
    data: *const c_void,
    size: usize,
    callback: RawFontReceive,
    config: *const ImFontConfig,
);

pub type RawFontResize = unsafe extern "C-unwind" fn(identifier: *const c_char, font_size: f32);

/// Registers a new callback to receive the font with the given identifier.
pub fn get_font(
    identifier: impl AsRef<str>,
    callback: RawFontReceive,
) -> Revertible<impl Fn() + Send + Sync + 'static> {
    let FontApi { get, release, .. } = AddonApi::get().font;
    let identifier = str_to_c(identifier, "failed to convert font identifier");
    unsafe { get(identifier.as_ptr(), callback) };
    let revert = move || unsafe { release(identifier.as_ptr(), callback) };
    revert.into()
}

/// Releases a previously registered callback for the font with the given identifier.
pub fn release_font(identifier: impl AsRef<str>, callback: RawFontReceive) {
    let FontApi { release, .. } = AddonApi::get().font;
    let identifier = str_to_c(identifier, "failed to convert font identifier");
    unsafe { release(identifier.as_ptr(), callback) }
}

/// Adds a font from a file path and sends updates to the callback.
pub fn add_font_from_file(
    identifier: impl AsRef<str>,
    file: impl AsRef<Path>,
    font_size: f32,
    config: Option<&ImFontConfig>,
    callback: RawFontReceive,
) -> Revertible<impl Fn() + Send + Sync + 'static> {
    let FontApi {
        add_from_file,
        release,
        ..
    } = AddonApi::get().font;
    let identifier = str_to_c(identifier, "failed to convert font identifier");
    let file = path_to_c(file, "failed to convert font file path");
    unsafe {
        add_from_file(
            identifier.as_ptr(),
            font_size,
            file.as_ptr(),
            callback,
            config.as_ptr_opt(),
        )
    };
    let revert = move || unsafe { release(identifier.as_ptr(), callback) };
    revert.into()
}

/// Adds a font from a resource and sends updates to the callback.
pub fn add_font_from_resource(
    identifier: impl AsRef<str>,
    handle: HMODULE,
    resource: u32,
    font_size: f32,
    config: Option<&ImFontConfig>,
    callback: RawFontReceive,
) -> Revertible<impl Fn() + Send + Sync + 'static> {
    let FontApi {
        add_from_resource,
        release,
        ..
    } = AddonApi::get().font;
    let identifier = str_to_c(identifier, "failed to convert font identifier");
    unsafe {
        add_from_resource(
            identifier.as_ptr(),
            font_size,
            resource,
            handle,
            callback,
            config.as_ptr_opt(),
        )
    };
    let revert = move || unsafe { release(identifier.as_ptr(), callback) };
    revert.into()
}

/// Adds a font from memory and sends updates to the callback.
pub fn add_font_from_memory(
    identifier: impl AsRef<str>,
    data: impl AsRef<[u8]>,
    font_size: f32,
    config: Option<&ImFontConfig>,
    callback: RawFontReceive,
) -> Revertible<impl Fn() + Send + Sync + 'static> {
    let FontApi {
        add_from_memory,
        release,
        ..
    } = AddonApi::get().font;
    let identifier = str_to_c(identifier, "failed to convert font identifier");
    let data = data.as_ref();
    unsafe {
        add_from_memory(
            identifier.as_ptr(),
            font_size,
            data.as_ptr().cast(),
            data.len(),
            callback,
            config.as_ptr_opt(),
        )
    };
    let revert = move || unsafe { release(identifier.as_ptr(), callback) };
    revert.into()
}

/// Macro to wrap a font receive callback.
///
/// Generates a [`RawFontReceive`] wrapper around the passed callback.
///
/// # Usage
/// ```no_run
/// # use nexus::font::*;
/// let font_receive: RawFontReceive = font_receive!(|id, font| {
///     use nexus::log::{log, LogLevel};
///     log(LogLevel::Info, "My Addon", format!("font {id} received"));
/// });
/// ```
#[macro_export]
macro_rules! font_receive {
    ( $callback:expr $(,)? ) => {{
        const __CALLBACK: fn(
            &::std::primitive::str,
            ::std::option::Option<&mut $crate::imgui::sys::ImFont>,
        ) = $callback;

        extern "C-unwind" fn __font_receive_wrapper(
            identifier: *const ::std::ffi::c_char,
            font: *mut $crate::imgui::sys::ImFont,
        ) {
            let identifier = unsafe { $crate::__macro::str_from_c(identifier) }
                .expect("invalid identifier in font callback");
            let font = unsafe { font.as_mut() };
            __CALLBACK(identifier, font)
        }

        __font_receive_wrapper
    }};
}

pub use font_receive;

/// Resizes an existing font, sending the update to registered callbacks.
pub fn resize_font(identifier: impl AsRef<str>, font_size: f32) {
    let FontApi { resize, .. } = AddonApi::get().font;
    let identifier = str_to_c(identifier, "failed to convert font identifier");
    unsafe { resize(identifier.as_ptr(), font_size) }
}
