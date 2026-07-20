//! FFI helpers.

#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{CStr, CString, NulError, OsString, c_char},
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
    ptr,
};
use windows::{
    Win32::{
        Globalization::{
            CP_ACP, CP_OEMCP, CP_UTF8, GetACP, MULTI_BYTE_TO_WIDE_CHAR_FLAGS, MultiByteToWideChar,
            WideCharToMultiByte,
        },
        Storage::FileSystem::AreFileApisANSI,
    },
    core::PCSTR,
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

/// Resolves the codepage to use for conversions.
pub fn get_codepage() -> u32 {
    if unsafe { GetACP() } == CP_UTF8 {
        CP_UTF8
    } else if !unsafe { AreFileApisANSI() }.as_bool() {
        CP_OEMCP
    } else {
        CP_ACP
    }
}

/// Attempts to convert an ANSI path string pointer to a [`PathBuf`].
#[inline]
pub fn path_from_ansi(ptr: *const c_char) -> Option<PathBuf> {
    path_from_ansi_with(ptr, get_codepage())
}

/// Attempts to convert an ANSI path string pointer to a [`PathBuf`] with the given codepage.
#[inline]
pub fn path_from_ansi_with(ptr: *const c_char, codepage: u32) -> Option<PathBuf> {
    if !ptr.is_null() {
        let narrow = unsafe { CStr::from_ptr(ptr) }.to_bytes();

        // attempt to convert to WTF-8
        let flags = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(0);
        let size = unsafe { MultiByteToWideChar(codepage, flags, narrow, None) };
        let mut wide = vec![0; size as usize];
        let written = unsafe { MultiByteToWideChar(codepage, flags, narrow, Some(&mut wide)) };
        (written == size).then(|| OsString::from_wide(&wide).into())
    } else {
        None
    }
}

/// Attempts to convert a string to a [`CString`].
#[inline]
pub fn path_to_ansi(path: impl AsRef<Path>) -> Option<CString> {
    path_to_ansi_with(path, get_codepage())
}

/// Attempts to convert a string to a [`CString`] with the given codepage.
#[inline]
pub fn path_to_ansi_with(path: impl AsRef<Path>, codepage: u32) -> Option<CString> {
    let path = path.as_ref();
    let wide = path.as_os_str().encode_wide().collect::<Vec<_>>();

    // attempt to convert to ANSI
    let flags = 0;
    let size = unsafe { WideCharToMultiByte(codepage, flags, &wide, None, PCSTR::null(), None) };
    let mut narrow = vec![0; size as usize];
    let written = unsafe {
        WideCharToMultiByte(
            codepage,
            flags,
            &wide,
            Some(&mut narrow),
            PCSTR::null(),
            None,
        )
    };
    (written == size).then(|| unsafe { CString::from_vec_unchecked(narrow) })
}

/// Helper trait to handle `Option<&T>`.
pub trait OptionRefExt<T> {
    /// Returns the string as [`T`] pointer or `null`.
    fn as_ptr_opt(&self) -> *const T;
}

impl<T> OptionRefExt<T> for Option<&T> {
    #[inline]
    fn as_ptr_opt(&self) -> *const T {
        self.map(|string| string as *const _).unwrap_or(ptr::null())
    }
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
