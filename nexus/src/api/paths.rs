use crate::{
    addon_api,
    util::{path_from_c, str_to_c},
    AddonApi,
};
use std::{ffi::c_char, path::PathBuf};

pub type RawGetGameDir = unsafe extern "C-unwind" fn() -> *const c_char;

pub type RawGetAddonDir = unsafe extern "C-unwind" fn(name: *const c_char) -> *const c_char;

pub type RawGetCommonDir = unsafe extern "C-unwind" fn() -> *const c_char;

/// Returns the game directory.
#[inline]
pub fn get_game_dir() -> Option<PathBuf> {
    let AddonApi { get_game_dir, .. } = addon_api();
    unsafe { path_from_c(get_game_dir()) }
}

/// Returns the directory for an addon with the passed name.
#[inline]
pub fn get_addon_dir(name: impl AsRef<str>) -> Option<PathBuf> {
    let AddonApi { get_addon_dir, .. } = addon_api();
    let name = str_to_c(name, "failed to convert addon dir name");
    unsafe { path_from_c(get_addon_dir(name.as_ptr())) }
}

/// Returns the common addon directory.
///
/// Synonymous with `get_addon_dir("common")`.
#[inline]
pub fn get_common_dir() -> Option<PathBuf> {
    let AddonApi { get_common_dir, .. } = addon_api();
    unsafe { path_from_c(get_common_dir()) }
}
