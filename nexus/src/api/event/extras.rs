//! [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) bridge events.

use super::Event;
use arcdps::{
    extras::{
        keybinds::RawKeybindChange, message::SquadMessage, user::to_user_info_iter, UserInfo,
        UserInfoIter,
    },
    Language,
};

/// Unofficial Extras squad update event.
pub const EXTRAS_SQUAD_UPDATE: Event<SquadUpdate> =
    unsafe { Event::new("EV_UNOFFICIAL_EXTRAS_SQUAD_UPDATE") };

/// Unofficial Extras squad update payload.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SquadUpdate {
    pub users: *const UserInfo,
    pub count: u64,
}

impl SquadUpdate {
    #[inline]
    pub fn iter(&self) -> UserInfoIter {
        unsafe { to_user_info_iter(self.users, self.count) }
    }
}

/// Unofficial Extras language changed event.
pub const LANGUAGE_CHANGED: Event<Language> =
    unsafe { Event::new("EV_UNOFFICIAL_EXTRAS_LANGUAGE_CHANGED") };

/// Unofficial Extras keybind changed event.
pub const KEYBIND_CHANGED: Event<RawKeybindChange> =
    unsafe { Event::new("EV_UNOFFICIAL_EXTRAS_KEYBIND_CHANGED") };

/// Unofficial Extras chat message event.
pub const CHAT_MESSAGE: Event<SquadMessage> =
    unsafe { Event::new("EV_UNOFFICIAL_EXTRAS_CHAT_MESSAGE") };
