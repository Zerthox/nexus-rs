use std::ffi::{c_char, c_void};

pub type RawDataGetResource =
    unsafe extern "C-unwind" fn(identifier: *const c_char) -> *const c_void;

pub type RawDataShareResource =
    unsafe extern "C-unwind" fn(identifier: *const c_char, resource_size: usize) -> *mut c_void;
