//! Windows [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc).

use crate::{revertible::Revertible, AddonApi, WndProcApi};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

pub type RawWndProcCallback =
    extern "C-unwind" fn(h_wnd: HWND, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> u32;

pub type RawWndProcAddRem = unsafe extern "C-unwind" fn(wnd_proc_callback: RawWndProcCallback);

pub type RawWndProcSendToGame = unsafe extern "C-unwind" fn(
    h_wnd: HWND,
    u_msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT;

/// Registers a new [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc) callback.
///
/// Returns a [`Revertible`] to revert the register.
pub fn register_wnd_proc(
    callback: RawWndProcCallback,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let WndProcApi {
        register,
        deregister,
        ..
    } = AddonApi::get().wnd_proc;
    unsafe { register(callback) };
    let revert = move || unsafe { deregister(callback) };
    revert.into()
}

/// Deregisters an already registered [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc) callback.
pub fn deregister_wnd_proc(callback: RawWndProcCallback) {
    let WndProcApi { deregister, .. } = AddonApi::get().wnd_proc;
    unsafe { deregister(callback) }
}

/// Sends a [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc) directly to the game, bypassing other hooks.
pub fn send_wnd_proc_to_game(h_wnd: HWND, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let WndProcApi {
        send_to_game_only, ..
    } = AddonApi::get().wnd_proc;
    unsafe { send_to_game_only(h_wnd, u_msg, w_param, l_param) }
}
