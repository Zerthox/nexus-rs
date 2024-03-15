pub mod addon;
pub mod api;

pub use self::addon::{AddonFlags, AddonLoad, AddonUnload, UpdateProvider};
pub use nexus_codegen::export;

use crate::api::AddonApi;
use std::ptr;

/// Fields supported by the [`export`] macro.
pub struct SupportedFields {
    /// Raidcore addon id or random unique negative integer, if not on Raidcore.
    pub signature: i32,

    /// Load function of the addon.
    pub load: Option<AddonLoad>,

    /// Unload function of the addon.
    pub unload: Option<AddonUnload>,

    /// Information about the addon.
    pub flags: Option<AddonFlags>,

    /// What platform the addon is hosted on.
    pub provider: Option<UpdateProvider>,

    /// Link to the update resource.
    pub update_link: Option<&'static str>,
}

pub(crate) static mut ADDON_API: Option<&AddonApi> = None;

pub(crate) static mut IMGUI_CTX: Option<imgui::Context> = None;

#[doc(hidden)]
pub mod __macro {
    use super::*;

    /// Initializes globals.
    pub unsafe fn init(api: *const AddonApi) {
        let api = api.as_ref().expect("no addon api supplied");
        ADDON_API = Some(api);

        // setup imgui
        imgui::sys::igSetCurrentContext(api.imgui_context);
        imgui::sys::igSetAllocatorFunctions(api.imgui_malloc, api.imgui_free, ptr::null_mut());
        IMGUI_CTX = Some(imgui::Context::current());

        // TODO: init logger
    }

    /// Creates an [`imgui::Ui`] for rendering a frame.
    pub unsafe fn ui<'a>() -> imgui::Ui<'a> {
        imgui::Ui::from_ctx(IMGUI_CTX.as_ref().unwrap_unchecked())
    }
}
