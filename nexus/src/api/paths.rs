use std::ffi::c_char;

pub type RawGetGameDir = unsafe extern "C-unwind" fn() -> *const c_char;

pub type RawGetAddonDir = unsafe extern "C-unwind" fn(name: *const c_char) -> *const c_char;

pub type RawGetCommonDir = unsafe extern "C-unwind" fn() -> *const c_char;
