#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{c_char, CStr, CString},
    path::{Path, PathBuf},
    ptr,
};

/// Helper to convert a C string pointer to a [`prim@str`].
#[inline]
pub unsafe fn str_from_c<'a>(ptr: *const c_char) -> Option<&'a str> {
    if !ptr.is_null() {
        CStr::from_ptr(ptr).to_str().ok()
    } else {
        None
    }
}

/// Helper to convert a C string pointer to a [`String`].
#[inline]
pub unsafe fn string_from_c(ptr: *const c_char) -> Option<String> {
    str_from_c(ptr).map(ToOwned::to_owned)
}

/// Helper to convert a C string pointer to a [`PathBuf`].
#[inline]
pub unsafe fn path_from_c(ptr: *const c_char) -> Option<PathBuf> {
    str_from_c(ptr).map(PathBuf::from)
}

/// Attempts to convert a string to a [`CString`].
/// Panics with the given error message if the string contains an internal nul byte.
#[inline]
pub fn str_to_c(string: impl AsRef<str>, err_msg: &str) -> CString {
    CString::new(string.as_ref()).expect(err_msg)
}

/// Attempts to convert a string to a [`CString`].
/// Panics with the given error message if the string contains an internal nul byte.
#[inline]
pub fn path_to_c(path: impl AsRef<Path>, err_msg: &str) -> CString {
    str_to_c(path.as_ref().to_str().expect(err_msg), err_msg)
}

/// Helper trait to handle `Option<&CStr>` and  `Option<CString>`.
pub trait OptionCStrExt {
    /// Returns the string as [`c_char`] pointer or `null`.
    #[allow(dead_code)]
    fn as_ptr_opt(&self) -> *const c_char;
}

impl OptionCStrExt for Option<&CStr> {
    #[inline]
    fn as_ptr_opt(&self) -> *const c_char {
        self.map(|string| string.as_ptr()).unwrap_or(ptr::null())
    }
}

impl OptionCStrExt for Option<CString> {
    #[inline]
    fn as_ptr_opt(&self) -> *const c_char {
        self.as_ref()
            .map(|string| string.as_ptr())
            .unwrap_or(ptr::null())
    }
}
