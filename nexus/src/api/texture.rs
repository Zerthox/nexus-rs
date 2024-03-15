use std::ffi::{c_char, c_void};
use windows::Win32::Foundation::HMODULE;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub resource: *const c_void,
}

pub type RawTextureReceiveCallback =
    unsafe extern "C-unwind" fn(identifier: *const c_char, texture: *const Texture);

pub type RawTextureGet = unsafe extern "C-unwind" fn(identifier: *const c_char) -> *const Texture;

pub type RawTextureGetOrCreateFromFile = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    filename: *const c_char,
) -> *const Texture;

pub type RawTextureGetOrCreateFromResource = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    resource_id: u32,
    module: HMODULE,
) -> *const Texture;

pub type RawTextureGetOrCreateFromUrl = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    remote: *const c_char,
    endpoint: *const c_char,
) -> *const Texture;

pub type RawTextureGetOrCreateFromMemory = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    data: *const c_void,
    size: usize,
) -> *const Texture;

pub type RawTextureLoadFromFile = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    filename: *const c_char,
    callback: RawTextureReceiveCallback,
);

pub type RawTextureLoadFromResource = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    resource_id: u32,
    module: HMODULE,
    callback: RawTextureReceiveCallback,
);

pub type RawTextureLoadFromUrl = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    remote: *const c_char,
    endpoint: *const c_char,
    callback: RawTextureReceiveCallback,
);

pub type RawTextureLoadFromMemory = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    data: *const c_void,
    size: usize,
    callback: RawTextureReceiveCallback,
);
