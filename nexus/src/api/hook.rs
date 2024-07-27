//! Hooking via [MinHook](https://github.com/TsudaKageyu/minhook).
//!
//! Enable the `"hook"` feature for bindings using trait interfaces from the [detour](https://github.com/darfink/detour-rs) crate.

use crate::{AddonApi, MinHookApi};
use std::{ffi::c_void, ptr};

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

impl HookStatus {
    /// Checks if the status is [`HookStatus::Ok`].
    #[inline]
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Converts the status to a [`Result`].
    #[inline]
    pub fn ok(self) -> Result<(), Self> {
        self.ok_then(())
    }

    /// Converts the status to a [`Result`] with a value.
    pub fn ok_then<T>(self, value: T) -> Result<T, Self> {
        match self {
            Self::Ok => Ok(value),
            _ => Err(self),
        }
    }
}

pub type RawHookCreate = unsafe extern "stdcall-unwind" fn(
    target: *const c_void,
    detour: *const c_void,
    trampoline: *mut *const c_void,
) -> HookStatus;

pub type RawHookRemove = unsafe extern "stdcall-unwind" fn(target: *const c_void) -> HookStatus;

pub type RawHookEnable = unsafe extern "stdcall-unwind" fn(target: *const c_void) -> HookStatus;

pub type RawHookDisable = unsafe extern "stdcall-unwind" fn(target: *const c_void) -> HookStatus;

/// Creates a hook for the specified target function in **disabled** state.
///
/// Returns a pointer to the trampoline function, which will be used to call the original target function.
///
/// # Safety
/// Target and detour must point to valid functions and have the same or a compatible function signature.
#[inline]
pub unsafe fn create_hook_raw(
    target: *const (),
    detour: *const (),
) -> Result<*const (), HookStatus> {
    let mut original = ptr::null();
    let MinHookApi { create, .. } = AddonApi::get().min_hook;
    let result = unsafe { create(target.cast(), detour.cast(), &mut original) };
    result.ok_then(original.cast())
}

/// Removes an already created hook.
#[inline]
pub fn remove_hook_raw(target: *const ()) -> Result<(), HookStatus> {
    let MinHookApi { remove, .. } = AddonApi::get().min_hook;
    let result = unsafe { remove(target.cast()) };
    result.ok()
}

/// Enables an already created hook.
#[inline]
pub fn enable_hook_raw(target: *const ()) -> Result<(), HookStatus> {
    let MinHookApi { enable, .. } = AddonApi::get().min_hook;
    let result = unsafe { enable(target.cast()) };
    result.ok()
}

/// Dsiables an already created hook.
#[inline]
pub fn disable_hook_raw(target: *const ()) -> Result<(), HookStatus> {
    let MinHookApi { disable, .. } = AddonApi::get().min_hook;
    let result = unsafe { disable(target.cast()) };
    result.ok()
}

#[cfg(feature = "hook")]
mod bindings {
    use super::*;
    use retour::{Function, HookableWith};

    /// Creates a hook for the specified target function in **disabled** state.
    ///
    /// Returns a pointer to the trampoline function, which will be used to call the original target function.
    pub fn create_hook<F, D>(target: F, detour: D) -> Result<*const (), HookStatus>
    where
        F: Function + HookableWith<D>,
        D: Function,
    {
        unsafe { create_hook_raw(target.to_ptr(), detour.to_ptr()) }
    }

    /// Creates a hook for the specified target function, and enables it if successful.
    ///
    /// Returns a pointer to the trampoline function, which will be used to call the original target function.
    pub fn create_hook_enabled<F, D>(target: F, detour: D) -> Result<*const (), HookStatus>
    where
        F: Function + HookableWith<D>,
        D: Function,
    {
        let trampoline = create_hook(target, detour)?;
        enable_hook(target)?;
        Ok(trampoline)
    }

    /// Removes an already created hook.
    pub fn remove_hook(target: impl Function) -> Result<(), HookStatus> {
        remove_hook_raw(target.to_ptr())
    }

    /// Enables an already created hook.
    pub fn enable_hook(target: impl Function) -> Result<(), HookStatus> {
        enable_hook_raw(target.to_ptr())
    }

    /// Disables an already created hook.
    pub fn disable_hook(target: impl Function) -> Result<(), HookStatus> {
        disable_hook_raw(target.to_ptr())
    }
}

#[cfg(feature = "hook")]
pub use bindings::*;
