use std::{
    ffi::{c_char, CStr},
    path::PathBuf,
};

/// Helper to convert a C string pointer to a [`prim@str`].
#[inline]
pub unsafe fn str_from_cstr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if !ptr.is_null() {
        CStr::from_ptr(ptr).to_str().ok()
    } else {
        None
    }
}

/// Helper to convert a C string pointer to a [`String`].
#[inline]
pub unsafe fn string_from_cstr(ptr: *const c_char) -> Option<String> {
    str_from_cstr(ptr).map(ToOwned::to_owned)
}

/// Helper to convert a C string pointer to a [`PathBuf`].
#[inline]
pub unsafe fn path_from_cstr(ptr: *const c_char) -> Option<PathBuf> {
    str_from_cstr(ptr).map(PathBuf::from)
}
