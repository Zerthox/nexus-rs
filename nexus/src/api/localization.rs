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
    let identifier = str_to_c(identifier, "failed to convert translation identifier");
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
    let identifier = str_to_c(identifier, "failed to convert translation identifier");
    let language = str_to_c(
        language_identifier,
        "failed to convert translation language identifier",
    );
    unsafe { string_from_c(translate_to(identifier.as_ptr(), language.as_ptr())) }
}

/// Attempts to set a translated string for the given identifier and language at runtime.
#[inline]
pub fn set_translation(
    identifier: impl AsRef<str>,
    language_identifier: impl AsRef<str>,
    string: impl AsRef<str>,
) {
    let LocalizationApi { set, .. } = AddonApi::get().localization;
    let identifier = str_to_c(identifier, "failed to convert translation identifier");
    let language = str_to_c(
        language_identifier,
        "failed to convert translation language identifier",
    );
    let string = str_to_c(string, "failed to convert translation string");
    unsafe { set(identifier.as_ptr(), language.as_ptr(), string.as_ptr()) }
}
