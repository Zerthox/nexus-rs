use std::ffi::c_char;

pub type LocalizationTranslate = unsafe extern "C-unwind" fn(identifier: *const c_char);

pub type LocalizationTranslateTo =
    unsafe extern "C-unwind" fn(identifier: *const c_char, language_identifier: *const c_char);
