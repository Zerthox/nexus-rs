//! Quick access creation.

use crate::{gui::RawGuiRender, revertible::Revertible, util::str_to_c, AddonApi, QuickAccessApi};
use std::{ffi::c_char, ptr};

pub type RawQuickAccessAddShortcut = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    texture_identifier: *const c_char,
    texture_hover_identifier: *const c_char,
    keybind_identifier: *const c_char,
    tooltip_text: *const c_char,
);

pub type RawQuickAccessAddContextMenu =
    unsafe extern "C-unwind" fn(identifier: *const c_char, shortcut_render_callback: RawGuiRender);

pub type RawQuickAccessAddContextMenu2 = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    target_identifier: *const c_char,
    shortcut_render_callback: RawGuiRender,
);

pub type RawQuickAccessGeneric = unsafe extern "C-unwind" fn(identifier: *const c_char);

// TODO: combination with texture & keybind calls

/// Adds a new shortcut icon to the quick access with the given texture identifiers.
/// When clicked the given keybind identifier is triggered.
///
/// Returns a [`Revertible`] to remove the shortcut.
pub fn add_quick_access(
    identifier: impl AsRef<str>,
    texture_identifier: impl AsRef<str>,
    texture_hover_identifier: impl AsRef<str>,
    keybind_identifier: impl AsRef<str>,
    tooltip_text: impl AsRef<str>,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let QuickAccessApi { add, remove, .. } = AddonApi::get().quick_access;
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
        add(
            identifier.as_ptr(),
            texture_identifier.as_ptr(),
            texture_hover_identifier.as_ptr(),
            keybind_identifier.as_ptr(),
            tooltip_text.as_ptr(),
        )
    };
    let revert = move || unsafe { remove(identifier.as_ptr()) };
    revert.into()
}

/// Removes a previously registered shortcut from the quick access.
pub fn remove_quick_access(identifier: impl AsRef<str>) {
    let QuickAccessApi { remove, .. } = AddonApi::get().quick_access;
    let identifier = str_to_c(identifier, "failed to convert shortcut identifier");
    unsafe { remove(identifier.as_ptr()) }
}

/// Sends a notification to the given quick access shortcut.
pub fn notify_quick_access(identifier: impl AsRef<str>) {
    let QuickAccessApi { notify, .. } = AddonApi::get().quick_access;
    let identifier = str_to_c(identifier, "failed to convert shortcut identifier");
    unsafe { notify(identifier.as_ptr()) }
}

/// Adds a new [`RawGuiRender`] callback for the shortcut context menu.
///
/// Returns a [`Revertible`] to remove the context menu.
pub fn add_quick_access_context_menu(
    identifier: impl AsRef<str>,
    target_identifier: Option<impl AsRef<str>>,
    render_callback: RawGuiRender,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let QuickAccessApi {
        add_context_menu,
        remove_context_menu,
        ..
    } = AddonApi::get().quick_access;
    let identifier = str_to_c(identifier, "failed to convert shortcut identifier");
    let target_identifier = target_identifier
        .map(|string| str_to_c(string, "failed to convert shortcut target identifier"));
    unsafe {
        add_context_menu(
            identifier.as_ptr(),
            target_identifier
                .as_ref()
                .map(|string| string.as_ptr())
                .unwrap_or(ptr::null()),
            render_callback,
        )
    };
    let revert = move || unsafe { remove_context_menu(identifier.as_ptr()) };
    revert.into()
}

/// Removes a previously registered shortcut context menu callback.
pub fn remove_quick_access_context_menu(identifier: impl AsRef<str>) {
    let QuickAccessApi {
        remove_context_menu,
        ..
    } = AddonApi::get().quick_access;
    let identifier = str_to_c(identifier, "failed to convert shortcut identifier");
    unsafe { remove_context_menu(identifier.as_ptr()) }
}

/// Adds a new shortcut icon to the quick access with the given texture identifiers.
/// When clicked the given keybind identifier is triggered.
///
/// Returns a [`Revertible`] to remove the shortcut.
#[deprecated = "use add_quick_access"]
pub fn add_shortcut(
    identifier: impl AsRef<str>,
    texture_identifier: impl AsRef<str>,
    texture_hover_identifier: impl AsRef<str>,
    keybind_identifier: impl AsRef<str>,
    tooltip_text: impl AsRef<str>,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    add_quick_access(
        identifier,
        texture_identifier,
        texture_hover_identifier,
        keybind_identifier,
        tooltip_text,
    )
}

/// Removes a previously registered shortcut from the quick access.
#[deprecated = "use remove_quick_access"]
pub fn remove_shortcut(identifier: impl AsRef<str>) {
    remove_quick_access(identifier)
}

/// Adds a new [`RawGuiRender`] callback fired when the quick access is right-clicked.
///
/// Returns a [`Revertible`] to remove the shortcut.
#[deprecated = "use add_quick_access_context_menu"]
pub fn add_simple_shortcut(
    identifier: impl AsRef<str>,
    render_callback: RawGuiRender,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    add_quick_access_context_menu(identifier, None::<&str>, render_callback)
}

/// Removes a previously registered simple shortcut callback.
#[deprecated = "use remove_quick_access_context_menu"]
pub fn remove_simple_shortcut(identifier: impl AsRef<str>) {
    remove_quick_access_context_menu(identifier)
}
