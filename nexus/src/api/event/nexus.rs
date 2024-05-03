//! Nexus events.

use super::Event;

/// Nexus addon loaded event.
pub const ADDON_LOADED: Event<i32> = unsafe { Event::new("EV_ADDON_LOADED") };

/// Nexus addon unloaded event.
pub const ADDON_UNLOADED: Event<i32> = unsafe { Event::new("EV_ADDON_UNLOADED") };

/// Nexus volatile addon disabled event.
pub const VOLATILE_ADDON_DISABLED: Event<i32> = unsafe { Event::new("EV_ADDON_LOADED") };

/// Window resized event.
pub const WINDOW_RESIZED: Event<()> = unsafe { Event::new("EV_WINDOW_RESIZED") };

/// Mumble identity updated event.
pub const MUMBLE_IDENTITY_UPDATED: Event<MumbleIdentityUpdate> =
    unsafe { Event::new("EV_MUMBLE_IDENTITY_UPDATED") };

/// Mumble identity.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct MumbleIdentityUpdate {
    pub name: [u8; 20],
    pub profession: u32,
    pub specialization: u32,
    pub race: u32,
    pub map_id: u32,
    pub world_id: u32,
    pub team_color_id: u32,
    pub is_commander: bool,
    pub fov: f32,
    pub ui_size: u32,
}
