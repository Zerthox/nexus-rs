use super::Event;
use std::ffi::c_void;

/// Unofficial Extras squad update event.
pub const EXTRAS_SQUAD_UPDATE: Event<SquadUpdate> =
    unsafe { Event::new("EV_UNOFFICIAL_EXTRAS_SQUAD_UPDATE") };

/// Unofficial Extras squad update payload.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SquadUpdate {
    pub users: *const c_void, // TODO: optional typing
    pub count: u64,
}

/// Unofficial Extras language changed event.
pub const LANGUAGE_CHANGED: Event<c_void> =
    unsafe { Event::new("EV_UNOFFICIAL_EXTRAS_LANGUAGE_CHANGED") };

/// Unofficial Extras keybind changed event.
pub const KEYBIND_CHANGED: Event<c_void> =
    unsafe { Event::new("EV_UNOFFICIAL_EXTRAS_KEYBIND_CHANGED") };

/// Unofficial Extras chat message event.
pub const CHAT_MESSAGE: Event<c_void> = unsafe { Event::new("EV_UNOFFICIAL_EXTRAS_CHAT_MESSAGE") };
