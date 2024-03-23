use crate::{
    addon_api,
    util::{path_to_c, str_to_c},
    AddonApi,
};
use std::{
    ffi::{c_char, c_void},
    path::Path,
};
use windows::Win32::Foundation::HMODULE;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub resource: *const c_void,
}

pub type RawTextureReceiveCallback =
    extern "C-unwind" fn(identifier: *const c_char, texture: *const Texture);

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

/// Attempts to retrieve a texture by its identifier.
pub fn get_texture(identifier: impl AsRef<str>) -> Option<Texture> {
    let AddonApi { get_texture, .. } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert texture identifier");
    unsafe { get_texture(identifier.as_ptr()).as_ref().cloned() }
}

/// Attempts to retrieve a texture or creates it from the given file path.
pub fn get_texture_or_create_from_file(
    identifier: impl AsRef<str>,
    file: impl AsRef<Path>,
) -> Option<Texture> {
    let AddonApi {
        get_texture_or_create_from_file,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert texture identifier");
    let file = path_to_c(file, "failed to convert texture file");
    unsafe {
        get_texture_or_create_from_file(identifier.as_ptr(), file.as_ptr())
            .as_ref()
            .cloned()
    }
}

/// Attempts to retrieve a texture or creates it from the given resource.
pub fn get_texture_or_create_from_resource(
    identifier: impl AsRef<str>,
    resource_id: u32,
    module: HMODULE,
) -> Option<Texture> {
    let AddonApi {
        get_texture_or_create_from_resource,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert texture identifier");
    unsafe {
        get_texture_or_create_from_resource(identifier.as_ptr(), resource_id, module)
            .as_ref()
            .cloned()
    }
}

/// Attempts to retrieve a texture or creates it from the given URL.
pub fn get_texture_or_create_from_url(
    identifier: impl AsRef<str>,
    remote: impl AsRef<str>,
    endpoint: impl AsRef<str>,
) -> Option<Texture> {
    let AddonApi {
        get_texture_or_create_from_url,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert texture identifier");
    let remote = str_to_c(remote, "failed to convert texture url remote");
    let endpoint = str_to_c(endpoint, "failed to convert texture url endpoint");
    unsafe {
        get_texture_or_create_from_url(identifier.as_ptr(), remote.as_ptr(), endpoint.as_ptr())
            .as_ref()
            .cloned()
    }
}

/// Attempts to retrieve a texture or creates it from the given memory.
pub fn get_texture_or_create_from_memory(
    identifier: impl AsRef<str>,
    memory: impl AsRef<[u8]>,
) -> Option<Texture> {
    let AddonApi {
        get_texture_or_create_from_memory,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert texture identifier");
    let memory = memory.as_ref();
    unsafe {
        get_texture_or_create_from_memory(identifier.as_ptr(), memory.as_ptr() as _, memory.len())
            .as_ref()
            .cloned()
    }
}

/// Loads a texture from the given file path.
pub fn load_texture_from_file_raw(
    identifier: impl AsRef<str>,
    file: impl AsRef<Path>,
    callback: Option<RawTextureReceiveCallback>,
) {
    let AddonApi {
        load_texture_from_file,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert texture identifier");
    let file = path_to_c(file, "foo");
    unsafe {
        load_texture_from_file(
            identifier.as_ptr(),
            file.as_ptr(),
            callback.unwrap_or(dummy_receive_texture),
        )
    }
}

/// Loads a texture from the given resource.
pub fn load_texture_from_resource_raw(
    identifier: impl AsRef<str>,
    resource_id: u32,
    module: HMODULE,
    callback: Option<RawTextureReceiveCallback>,
) {
    let AddonApi {
        load_texture_from_resource,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert texture identifier");
    unsafe {
        load_texture_from_resource(
            identifier.as_ptr(),
            resource_id,
            module,
            callback.unwrap_or(dummy_receive_texture),
        )
    }
}

/// Loads a texture from the given URL.
///
/// # Usage
/// ```no_run
/// # use nexus::texture::*;
/// # extern "C-unwind" fn receive_texture(_identifier: *const std::ffi::c_char, _texture: *const Texture) {}
/// load_texture_from_url_raw(
///     "TEX_DUNGEON_ICON",
///     "https://render.guildwars2.com",
///     "/file/943538394A94A491C8632FBEF6203C2013443555/102478.png",
///     Some(receive_texture),
/// )
/// ```
pub fn load_texture_from_url_raw(
    identifier: impl AsRef<str>,
    remote: impl AsRef<str>,
    endpoint: impl AsRef<str>,
    callback: Option<RawTextureReceiveCallback>,
) {
    let AddonApi {
        load_texture_from_url,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert texture identifier");
    let remote = str_to_c(remote, "failed to convert texture url remote");
    let endpoint = str_to_c(endpoint, "failed to convert texture url endpoint");
    unsafe {
        load_texture_from_url(
            identifier.as_ptr(),
            remote.as_ptr(),
            endpoint.as_ptr(),
            callback.unwrap_or(dummy_receive_texture),
        )
    }
}

/// Loads a texture from the given memory.
pub fn load_texture_from_memory_raw(
    identifier: impl AsRef<str>,
    memory: impl AsRef<[u8]>,
    callback: Option<RawTextureReceiveCallback>,
) {
    let AddonApi {
        load_texture_from_memory,
        ..
    } = addon_api();
    let identifier = str_to_c(identifier, "failed to convert texture identifier");
    let memory = memory.as_ref();
    unsafe {
        load_texture_from_memory(
            identifier.as_ptr(),
            memory.as_ptr() as _,
            memory.len(),
            callback.unwrap_or(dummy_receive_texture),
        )
    }
}

extern "C-unwind" fn dummy_receive_texture(_identifier: *const c_char, _texture: *const Texture) {}
