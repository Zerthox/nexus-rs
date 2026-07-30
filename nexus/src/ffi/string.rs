use std::{
    ffi::{CStr, CString, NulError, c_char},
    ptr,
};

/// Attempts to convert a C string pointer to a [`prim@str`].
#[inline]
pub unsafe fn str_from_c<'a>(ptr: *const c_char) -> Option<&'a str> {
    if !ptr.is_null() {
        unsafe { CStr::from_ptr(ptr) }.to_str().ok()
    } else {
        None
    }
}

/// Attempts to convert a C string pointer to a [`String`].
#[inline]
pub unsafe fn string_from_c(ptr: *const c_char) -> Option<String> {
    unsafe { str_from_c(ptr) }.map(ToOwned::to_owned)
}

/// Attempts to convert a string to a [`CString`].
#[inline]
pub fn str_to_c(string: impl AsRef<str>) -> Result<CString, NulError> {
    CString::new(string.as_ref())
}

/// Helper trait to handle `Option<&CStr>` and  `Option<CString>`.
pub trait OptionCStrExt {
    /// Returns the string as [`c_char`] pointer or `null`.
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
