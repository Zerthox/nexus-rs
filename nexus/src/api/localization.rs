//! Localization of strings.

use crate::{
    util::{str_to_c, string_from_c},
    AddonApi, LocalizationApi,
};
use std::ffi::c_char;

pub type RawLocalizationTranslate =
    unsafe extern "C-unwind" fn(identifier: *const c_char) -> *const c_char;

pub type RawLocalizationTranslateTo = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    language_identifier: *const c_char,
) -> *const c_char;

pub type RawLocalizationSet = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    language_identifier: *const c_char,
    string: *const c_char,
);

/// Attempts to translate the identifier into the current active language.
/// Returns the same identifier if not available.
#[inline]
pub fn translate(identifier: impl AsRef<str>) -> Option<String> {
    let LocalizationApi { translate, .. } = AddonApi::get().localization;
    let identifier = str_to_c(identifier, "failed to convert translate identifier");
    unsafe { string_from_c(translate(identifier.as_ptr())) }
}

/// Attempts to translate the identifier into the given language.
/// Returns the same identifier if not available.
#[inline]
pub fn translate_to(
    identifier: impl AsRef<str>,
    language_identifier: impl AsRef<str>,
) -> Option<String> {
    let LocalizationApi { translate_to, .. } = AddonApi::get().localization;
    let identifier = str_to_c(identifier, "failed to convert translate identifier");
    let language = str_to_c(
        language_identifier,
        "failed to convert translate language identifier",
    );
    unsafe { string_from_c(translate_to(identifier.as_ptr(), language.as_ptr())) }
}
