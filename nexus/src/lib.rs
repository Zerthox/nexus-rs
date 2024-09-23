//! Bindings for Raidcore Nexus addons.
//!
//! # Usage
//! ```no_run
//! # mod main {
//! use nexus::{
//!     gui::{register_render, render, RenderType},
//!     imgui::Window
//! };
//!
//! nexus::export! {
//!     name: "My Addon",
//!     signature: -0x12345678,
//!     load: || {
//!         register_render(RenderType::Render, render!(|ui| {
//!             Window::new("My Window").build(ui, || {
//!                 ui.text("Hello World");
//!             });
//!         }));
//!     }
//! }
//! # }
//! ```

pub mod addon;
mod api;
mod globals;
mod revertible;
mod util;

#[cfg(feature = "log")]
mod logger;

pub use self::{
    addon::{AddonFlags, AddonLoad, AddonUnload, UpdateProvider},
    api::*,
    globals::{on_unload, ui},
    revertible::Revertible,
};
pub use imgui;
pub use nexus_codegen::export;

/// Returns the Nexus [`AddonApi`] instance.
///
/// Panics if called before initialization.
#[inline]
#[deprecated = "use AddonApi::get() instead"]
pub fn addon_api() -> &'static AddonApi {
    AddonApi::get()
}

/// Fields supported by the [`export`] macro.
pub struct SupportedFields {
    /// Raidcore addon id or random unique negative integer, if not on Raidcore.
    pub signature: i32,

    /// Name of the addon. Defaults to `CARGO_PKG_NAME`.
    pub name: Option<String>,

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

    #[cfg(feature = "log")]
    /// Filter for the log. Same syntax as [env_logger](https://docs.rs/env_logger/latest/env_logger/#enabling-logging).
    pub log_filter: Option<&'static str>,
}

#[doc(hidden)]
pub mod __macro {
    pub use crate::{
        globals::{deinit, init},
        util::str_from_c,
    };
}
