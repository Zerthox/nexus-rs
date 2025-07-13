//! [ImGui](https://github.com/ocornut/imgui) rendering via [`imgui-rs`](crate::imgui).

use crate::{util::str_to_c, AddonApi, RendererApi, Revertible, UiApi};
use std::ffi::{c_char, c_void};

/// ImGui version.
// TODO: is this still correct?
pub const IMGUI_VERSION: u32 = 18000;

/// Type of render callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIter,
        strum::IntoStaticStr,
        strum::VariantArray,
        strum::VariantNames
    )
)]
#[repr(C)]
pub enum RenderType {
    /// Before ImGui frame is initialized.
    PreRender,

    /// During ImGui frame.
    Render,

    /// After ImGui frame was ended.
    PostRender,

    /// During ImGui frame, appended to options window.
    OptionsRender,
}

pub type RawGuiRender = extern "C-unwind" fn();

pub type RawGuiAddRender =
    unsafe extern "C-unwind" fn(render_type: RenderType, render_callback: RawGuiRender);

pub type RawGuiRemRender = unsafe extern "C-unwind" fn(render_callback: RawGuiRender);

pub type RawGuiRegisterCloseOnEscape =
    unsafe extern "C-unwind" fn(window_name: *const c_char, is_visible: *mut bool);

pub type RawGuiDeregisterCloseOnEscape = unsafe extern "C-unwind" fn(window_name: *const c_char);

pub type ImguiMalloc = unsafe extern "C" fn(size: usize, user_data: *mut c_void) -> *mut c_void;

pub type ImguiFree = unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void);

/// Registers a new ImGui render callback of the given [`RenderType`].
///
/// Returns a [`Revertible`] to revert the register.
///
/// # Usage
/// ```no_run
/// # use nexus::gui::*;
/// let render_callback = render!(|ui| ui.text("Hello World"));
/// register_render(RenderType::Render, render_callback).revert_on_unload();
/// ```
#[inline]
pub fn register_render(
    render_type: RenderType,
    callback: RawGuiRender,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let RendererApi {
        register,
        deregister,
        ..
    } = AddonApi::get().renderer;
    unsafe { register(render_type, callback) };
    let revert = move || unsafe { deregister(callback) };
    revert.into()
}

/// Unregisters a previously registered ImGui render callback.
#[inline]
pub fn unregister_render(callback: RawGuiRender) {
    let RendererApi { deregister, .. } = AddonApi::get().renderer;
    unsafe { deregister(callback) }
}

/// Macro to wrap an ImGui render callback.
///
/// Generates a [`RawGuiRender`] wrapper around the passed callback.
///
/// # Usage
/// ```no_run
/// # use nexus::gui::*;
/// let render_callback: RawGuiRender = render!(|ui| ui.text("Hello World"));
/// ```
#[macro_export]
macro_rules! render {
    ( $callback:expr $(,)? ) => {{
        const __CALLBACK: fn(&$crate::imgui::Ui) = $callback;

        extern "C-unwind" fn __render_callback_wrapper() {
            unsafe { $crate::__macro::with_ui(__CALLBACK) }
        }

        __render_callback_wrapper
    }};
}

pub use render;

/// Registers a window name to get its bool toggled when escape is pressed.
pub fn register_close_on_escape(window_name: impl AsRef<str>, opened: &mut bool) {
    let UiApi {
        register_close_on_escape,
        ..
    } = AddonApi::get().ui;
    let window_name = str_to_c(window_name, "failed to convert window name");
    unsafe { register_close_on_escape(window_name.as_ptr(), opened) }
}

///  Deregisters a window name to listen to on escape.
pub fn deregister_close_on_escape(window_name: impl AsRef<str>) {
    let UiApi {
        deregister_close_on_escape,
        ..
    } = AddonApi::get().ui;
    let window_name = str_to_c(window_name, "failed to convert window name");
    unsafe { deregister_close_on_escape(window_name.as_ptr()) }
}
