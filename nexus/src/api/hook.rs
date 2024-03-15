use std::ffi::c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum HookStatus {
    Unknown = -1,
    Ok = 0,
    ErrorAlreadyInitialized,
    ErrorNotInitialized,
    ErrorAlreadyCreated,
    ErrorNotCreated,
    ErrorEnabled,
    ErrorDisabled,
    ErrorNotExecutable,
    ErrorUnsupportedFunction,
    ErrorMemoryAlloc,
    ErrorMemoryProtect,
    ErrorModuleNotFound,
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
