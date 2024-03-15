use std::ffi::c_char;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

pub type RawWndProcCallback =
    unsafe extern "C-unwind" fn(h_wnd: HWND, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> u32;

pub type RawWndProcAddRem = unsafe extern "C-unwind" fn(wnd_proc_callback: RawWndProcCallback);

pub type RawWndProcSendToGame = unsafe extern "C-unwind" fn(
    h_wnd: HWND,
    u_msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Keybind {
    pub key: u16,
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
}

pub type RawKeybindHandler = unsafe extern "C-unwind" fn(identifier: *const c_char);

pub type RawKeybindRegisterWithString = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    keybind_handler: RawKeybindHandler,
    keybind: *const c_char,
);

pub type RawKeybindRegisterWithStruct = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    keybind_handler: RawKeybindHandler,
    keybind: Keybind,
);

pub type RawKeybindDeregister = unsafe extern "C-unwind" fn(identifier: *const c_char);
