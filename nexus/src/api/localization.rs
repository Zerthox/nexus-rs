use crate::{addon_api, util::string_from_cstr, AddonApi};
use std::ffi::{c_char, CString};

pub type RawLocalizationTranslate =
    unsafe extern "C-unwind" fn(identifier: *const c_char) -> *const c_char;

pub type RawLocalizationTranslateTo = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    language_identifier: *const c_char,
) -> *const c_char;

/// Attempts to translate the identifier into the current active language.
/// Returns the same identifier if not available.
pub fn translate(identifier: impl AsRef<str>) -> Option<String> {
    let AddonApi { translate, .. } = addon_api();
    let identifier =
        CString::new(identifier.as_ref()).expect("failed to convert translate identifier");
    unsafe { string_from_cstr(translate(identifier.as_ptr())) }
}

/// Attempts to translate the identifier into the given language.
/// Returns the same identifier if not available.
pub fn translate_to(
    identifier: impl AsRef<str>,
    language_identifier: impl AsRef<str>,
) -> Option<String> {
    let AddonApi { translate_to, .. } = addon_api();
    let identifier =
        CString::new(identifier.as_ref()).expect("failed to convert translate identifier");
    let language = CString::new(language_identifier.as_ref())
        .expect("failed to convert translate language identifier");
    unsafe { string_from_cstr(translate_to(identifier.as_ptr(), language.as_ptr())) }
}
