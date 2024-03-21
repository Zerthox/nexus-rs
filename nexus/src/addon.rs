use bitflags::bitflags;
use std::ffi::c_char;

use crate::api::AddonApi;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AddonDefinition {
    /// Raidcore addon id or random unique negative integer, if not on Raidcore.
    pub signature: i32,

    /// Determines which [`AddonApi`] struct revision the Loader will pass.
    pub api_version: i32,

    /// Name of the addon.
    pub name: *const c_char,

    /// Version of the addon.
    pub version: AddonVersion,

    /// Author of the addon.
    pub author: *const c_char,

    /// Short addon description.
    pub description: *const c_char,

    /// Load function of the addon.
    pub load: RawAddonLoad,

    /// Unload function of the addon.
    ///
    /// Not required if [`AddonFlags::DisableHotloading`] is set.
    pub unload: Option<RawAddonUnload>,

    /// Information about the addon
    pub flags: AddonFlags,

    /// What platform is the the addon hosted on.
    pub provider: UpdateProvider,

    /// Link to the update resource.
    pub update_link: *const c_char,
}

unsafe impl Send for AddonDefinition {}

unsafe impl Sync for AddonDefinition {}

pub type AddonLoad = fn();

pub type AddonUnload = fn();

pub type RawAddonLoad = unsafe extern "C-unwind" fn(api: *const AddonApi);

pub type RawAddonUnload = unsafe extern "C-unwind" fn();

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AddonVersion {
    pub major: i16,
    pub minor: i16,
    pub build: i16,
    pub revision: i16,
}

bitflags! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AddonFlags: u32 {
        const None = 0;

        /// Hooking functions or doing anything else that is volatile and game build dependant.
        const IsVolatile = 1;

        /// Prevents unloading at runtime. Will require a restart if updated, etc.
        const DisableHotloading = 2;
    }
}

// TODO: rust enum encapsulating provider & link?
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum UpdateProvider {
    /// Does not support auto updating.
    None = 0,

    /// Raidcore via API.
    Raidcore = 1,

    /// GitHub releases.
    GitHub = 2,

    /// Direct file link.
    Direct = 3,
}
