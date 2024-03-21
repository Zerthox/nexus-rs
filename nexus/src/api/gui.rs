use crate::{addon_api, ui};
use imgui::Ui;
use std::{ffi::c_void, sync::Mutex};

/// ImGui version.
// TODO: is this still correct?
pub const IMGUI_VERSION: u32 = 18000;

/// Type of render callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

pub type RawGuiRender = unsafe extern "C-unwind" fn();

pub type RawGuiAddRender =
    unsafe extern "C-unwind" fn(render_type: RenderType, render_callback: RawGuiRender);

pub type RawGuiRemRender = unsafe extern "C-unwind" fn(render_callback: RawGuiRender);

pub type ImguiMalloc = unsafe extern "C" fn(size: usize, user_data: *mut c_void) -> *mut c_void;

pub type ImguiFree = unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void);

type RenderCallback = Box<dyn FnMut(&Ui) + Send + 'static>;

macro_rules! define_render_wrappers {
    ($( $name:ident: $type:expr ),*) => {
        $( mod $name {
            use super::*;

            static RENDER_CALLBACK: Mutex<Option<RenderCallback>> = Mutex::new(None);

            unsafe extern "C-unwind" fn render_wrapper() {
                let mut guard = RENDER_CALLBACK.lock().unwrap();
                let callback = guard
                    .as_mut()
                    .expect("attempt to call non-existent render callback");
                let ui = ui();
                callback(ui);
            }

            pub fn register(callback: impl FnMut(&Ui) + Send + 'static) {
                let mut render_callback = RENDER_CALLBACK.lock().unwrap();
                if render_callback.is_none() {
                    let register = addon_api().register_render;
                    unsafe { register($type, render_wrapper) }
                }
                *render_callback = Some(Box::new(callback));
            }

            pub fn unregister() {
                if RENDER_CALLBACK.lock().unwrap().take().is_some() {
                    let deregister = addon_api().deregister_render;
                    unsafe { deregister(render_wrapper) }
                }
            }
        } )*
    };
}

define_render_wrappers! {
    pre_render: RenderType::PreRender,
    render: RenderType::Render,
    post_render: RenderType::PostRender,
    options_render: RenderType::OptionsRender
}

/// Registers the ImGui render callback of the given [`RenderType`].
///
/// **Important:** currently this function only supports one callback for each type.
/// Adding another callback of the same type will overwrite the previous one.
#[inline]
pub fn register_render(render_type: RenderType, callback: impl FnMut(&Ui) + Send + 'static) {
    match render_type {
        RenderType::PreRender => pre_render::register(callback),
        RenderType::Render => render::register(callback),
        RenderType::PostRender => post_render::register(callback),
        RenderType::OptionsRender => options_render::register(callback),
    }
}

/// Unregisters the ImGui render callback of the given [`RenderType`].
///
/// Noop if no callback was registered for the given type.
#[inline]
pub fn unregister_render(render_type: RenderType) {
    match render_type {
        RenderType::PreRender => pre_render::unregister(),
        RenderType::Render => render::unregister(),
        RenderType::PostRender => post_render::unregister(),
        RenderType::OptionsRender => options_render::unregister(),
    }
}

/// Unregisters all ImGui render callbacks.
#[inline]
pub fn unregister_all() {
    pre_render::unregister();
    render::unregister();
    post_render::unregister();
    options_render::unregister();
}
