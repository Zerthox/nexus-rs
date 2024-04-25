//! Quick access creation.

use crate::{addon_api, gui::RawGuiRender, revertible::Revertible, util::str_to_c, AddonApi};
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

// TODO: combination with texture & keybind calls

/// Adds a new shortcut icon to the quick access with the given texture identifiers.
/// When clicked the given keybind identifier is triggered.
///
/// Returns a [`Revertible`] to remove the shortcut.
pub fn add_shortcut(
    identifier: impl AsRef<str>,
    texture_identifier: impl AsRef<str>,
    texture_hover_identifier: impl AsRef<str>,
    keybind_identifier: impl AsRef<str>,
    tooltip_text: impl AsRef<str>,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let AddonApi {
        add_shortcut,
        remove_shortcut,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert shortcut identifier");
    let texture_identifier = str_to_c(
        texture_identifier,
        "failed to convert shortcut texture identifier",
    );
    let texture_hover_identifier = str_to_c(
        texture_hover_identifier,
        "failed to convert shortcut hover texture identifier",
    );
    let keybind_identifier = str_to_c(
        keybind_identifier,
        "failed to convert shortcut keybind identifier",
    );
    let tooltip_text = str_to_c(tooltip_text, "failed to convert shortcut tooltip text");
    unsafe {
        add_shortcut(
            identifier.as_ptr(),
            texture_identifier.as_ptr(),
            texture_hover_identifier.as_ptr(),
            keybind_identifier.as_ptr(),
            tooltip_text.as_ptr(),
        )
    };
    let revert = move || unsafe { remove_shortcut(identifier.as_ptr()) };
    revert.into()
}

/// Removes a previously registered shortcut from the quick access.
pub fn remove_shortcut(identifier: impl AsRef<str>) {
    let AddonApi {
        remove_shortcut, ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert shortcut identifier");
    unsafe { remove_shortcut(identifier.as_ptr()) }
}

/// Adds a new [`RawGuiRender`] callback fired when the quick access is right-clicked.
///
/// Returns a [`Revertible`] to remove the shortcut.
pub fn add_simple_shortcut(
    identifier: impl AsRef<str>,
    render_callback: RawGuiRender,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let AddonApi {
        add_simple_shortcut,
        remove_simple_shortcut,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert simple shortcut identifier");
    unsafe { add_simple_shortcut(identifier.as_ptr(), render_callback) };
    let revert = move || unsafe { remove_simple_shortcut(identifier.as_ptr()) };
    revert.into()
}

/// Removes a previously registered simple shortcut callback.
pub fn remove_simple_shortcut(identifier: impl AsRef<str>) {
    let AddonApi {
        remove_simple_shortcut,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert simple shortcut identifier");
    unsafe { remove_simple_shortcut(identifier.as_ptr()) }
}
