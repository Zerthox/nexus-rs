//! Hooking via [MinHook](https://github.com/TsudaKageyu/minhook).
//!
//! Enable the `"hook"` feature for bindings using trait interfaces from the [detour](https://github.com/darfink/detour-rs) crate.

use std::ffi::c_void;

/// MinHook error codes.
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
pub enum HookStatus {
    /// Unknown error. Should not be returned.
    Unknown = -1,

    /// Successful.
    Ok = 0,

    /// MinHook is already initialized.
    ErrorAlreadyInitialized,

    /// MinHook is not initialized yet, or already uninitialized.
    ErrorNotInitialized,

    /// The hook for the specified target function is already created.
    ErrorAlreadyCreated,

    /// The hook for the specified target function is not created yet.
    ErrorNotCreated,

    /// The hook for the specified target function is already enabled.
    ErrorEnabled,

    /// The hook for the specified target function is not enabled yet, or already disabled.
    ErrorDisabled,

    /// The specified pointer is invalid.
    /// It points the address of non-allocated and/or non-executable region.
    ErrorNotExecutable,

    /// The specified target function cannot be hooked.
    ErrorUnsupportedFunction,

    /// Failed to allocate memory.
    ErrorMemoryAlloc,

    /// Failed to change the memory protection.
    ErrorMemoryProtect,

    /// The specified module is not loaded.
    ErrorModuleNotFound,

    /// The specified function is not found.
    ErrorFunctionNotFound,
}

pub type RawHookCreate = unsafe extern "stdcall-unwind" fn(
    target: *const c_void,
    detour: *const c_void,
    original: *mut *const c_void,
) -> HookStatus;

pub type RawHookRemove = unsafe extern "stdcall-unwind" fn(target: *const c_void) -> HookStatus;

pub type RawHookEnable = unsafe extern "stdcall-unwind" fn(target: *const c_void) -> HookStatus;

pub type RawHookDisable = unsafe extern "stdcall-unwind" fn(target: *const c_void) -> HookStatus;

#[cfg(feature = "hook")]
mod bindings {
    use super::*;
    use crate::addon_api;
    use detour::{Function, HookableWith};
    use std::ptr;

    /// Creates a hook for the specified target function, in **disabled** state.
    ///
    /// Returns a pointer to the trampoline function, which will be used to call the original target function.
    #[inline]
    pub fn create_hook<F, D>(target: F, detour: D) -> Result<*const (), HookStatus>
    where
        F: Function + HookableWith<D>,
        D: Function,
    {
        let mut original = ptr::null();
        let create = addon_api().hook_create;
        let err = unsafe {
            create(
                target.to_ptr().cast(),
                detour.to_ptr().cast(),
                &mut original,
            )
        };
        match err {
            HookStatus::Ok => Ok(original.cast()),
            _ => Err(err),
        }
    }

    /// Creates a hook for the specified target function, and enables it if successful.
    ///
    /// Returns a pointer to the trampoline function, which will be used to call the original target function.
    #[inline]
    pub fn create_hook_enabled<F, D>(target: F, detour: D) -> Result<*const (), HookStatus>
    where
        F: Function + HookableWith<D>,
        D: Function,
    {
        let result = create_hook(target, detour);
        if result.is_ok() {
            enable_hook(target);
        }
        result
    }

    /// Removes an already created hook.
    #[inline]
    pub fn remove_hook(target: impl Function) -> HookStatus {
        let remove = addon_api().hook_remove;
        unsafe { remove(target.to_ptr().cast()) }
    }

    /// Enables an already created hook.
    #[inline]
    pub fn enable_hook(target: impl Function) -> HookStatus {
        let enable = addon_api().hook_enable;
        unsafe { enable(target.to_ptr().cast()) }
    }

    /// Disables an already created hook.
    #[inline]
    pub fn disable_hook(target: impl Function) -> HookStatus {
        let disable = addon_api().hook_disable;
        unsafe { disable(target.to_ptr().cast()) }
    }
}

#[cfg(feature = "hook")]
pub use bindings::*;
