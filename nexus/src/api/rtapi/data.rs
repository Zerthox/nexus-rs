use super::{Attributes, GroupMemberVitals};
use crate::data_link::{get_resource, read_resource};
use bitfields::bitfield;
use std::ffi::c_char;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RealTimeData {
    /// Game build number.
    pub game_build: u32,

    /// Current game state.
    pub game_state: u32,

    /// Language setting in game client.
    pub language: u32,

    /// Tyrian time of day.
    pub time_of_day: u32,

    /// Map id of current map.
    pub map_id: u32,

    /// Map type of current map.
    pub map_type: u32,

    /// IP address of current server.
    pub ip_address: [u8; 4],

    /// Location of cursor in the game world as ingame coordinates.
    pub cursor: [f32; 3],

    /// Locations of squad markers in the game world as ingame coordinates.
    pub squad_markers: [[f32; 3]; 8],

    /// Type of current group.
    pub group_type: u32,

    /// Number of members in current group.
    pub group_member_count: u32,

    /// Reserved for future use.
    _reserved2: u32,

    /// Size of the struct as written by RTAPI.
    ///
    /// 0 on builds before this field existed.
    /// Check this before reading appended fields.
    pub size: u32,

    /// Account name of current player.
    pub account_name: [c_char; 140],

    /// Character name of current player.
    pub character_name: [c_char; 140],

    /// Current position of character.
    pub character_position: [f32; 3],

    /// Current facing direction of character.
    pub character_facing: [f32; 3],

    /// Profession of character.
    pub profession: u32,

    /// Current 3rd specialization of character.
    pub elite_specialization: u32,

    /// Index of the mount, if applicable.
    pub mount_index: u32,

    /// Current state of character.
    pub character_state: u32,

    /// Camera position in the game world.
    pub camera_position: [f32; 3],

    /// Camera facing direction.
    pub camera_facing: [f32; 3],

    /// Camera field of view.
    pub camera_fov: f32,

    /// Whether action camera is enabled.
    pub is_action_camera: IsActionCam,

    /// Character Level
    pub character_level: u32,
    /// Effective Character Level (Scaling)
    pub character_effective_level: u32,

    /// 0-based build template tab.
    ///
    /// The full build is in the [`RTAPI_BUILD_CHANGED`](super::event::RTAPI_BUILD_CHANGED) event.
    pub build_template_index: u32,

    /// 0-based equipment template tab.
    ///
    /// The full equipment is in the [`RTAPI_GEAR_CHANGED`](super::event::RTAPI_GEAR_CHANGED) event.
    pub gear_template_index: u32,

    /// Attributes from level and equipment, before scaling and effects.
    pub base_attributes: Attributes,

    /// Attributes as the hero panel shows them, scaled and including boons such as Might.
    pub effective_attributes: Attributes,

    /// Agent id of the target, same as in the combat events. 0 if nothing is selected.
    pub target_id: u32,

    /// Agent type of the target, see [`AgentType`](super::AgentType).
    pub target_type: u32,

    /// Species id for characters, gadget id for gadgets and attack targets.
    ///
    /// Same as [`CombatAgent::species_id`](super::CombatAgent::species_id).
    pub target_species_id: u32,

    /// Attitude of the target, see [`Attitude`](super::Attitude).
    pub target_attitude: u32,

    /// Health of the target in percent, 0-100.
    ///
    /// Currently whole numbers, as the game shows them.
    pub target_health: f32,

    /// Barrier of the target in percent of max health, 0-100.
    ///
    /// Currently whole numbers, as the game shows them.
    pub target_barrier: f32,

    /// Breakbar state of the target, see [`BreakbarState`](super::BreakbarState).
    pub target_breakbar_state: u32,

    /// Breakbar of the target in percent, 0-100.
    ///
    /// Currently whole numbers, as the game shows them.
    pub target_breakbar: f32,

    /// Id of the represented guild, empty if not representing a guild.
    pub represented_guild_id: [c_char; 40],

    /// Name of the represented guild in UTF-8.
    pub represented_guild_name: [c_char; 140],

    /// Frame counter.
    ///
    /// Odd while RTAPI writes a frame, even when it is complete.
    pub frame: u32,
}

impl RealTimeData {
    /// Signature of the RealTime API addon.
    pub const SIG: i32 = 0x2501A02C;

    /// RealTime API data link identifier.
    pub const LINK: &str = "RTAPI";

    #[inline]
    pub fn get_ptr() -> *const Self {
        get_resource(Self::LINK)
    }

    #[inline]
    pub fn read() -> Option<Self> {
        unsafe { read_resource(Self::LINK) }
    }
}

/// Vitals of the current group.
///
/// Updated every frame, synchronized via [`frame`](RealTimeGroupData::frame) like [`RealTimeData`].
#[derive(Debug, Clone)]
#[repr(C)]
pub struct RealTimeGroupData {
    /// Size of the struct as written by RTAPI. 0 when RTAPI is unloaded.
    pub size: u32,

    /// Number of used entries in [`members`](RealTimeGroupData::members).
    pub count: u32,

    /// Member vitals, only the first [`count`](RealTimeGroupData::count) entries are used.
    pub members: [GroupMemberVitals; 50],

    /// Frame counter.
    ///
    /// Odd while RTAPI writes a frame, even when it is complete.
    pub frame: u32,
}

impl RealTimeGroupData {
    /// RealTime API group data link identifier.
    pub const LINK: &str = "RTAPI_GROUP";

    #[inline]
    pub fn get_ptr() -> *const Self {
        get_resource(Self::LINK)
    }

    #[inline]
    pub fn read() -> Option<Self> {
        unsafe { read_resource(Self::LINK) }
    }

    /// Returns the used member entries.
    #[inline]
    pub fn members(&self) -> &[GroupMemberVitals] {
        let count = (self.count as usize).min(self.members.len());
        &self.members[..count]
    }
}

// This only has a single bit, so maybe just using u32 is enough, since this is only read I assume
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsActionCam {
    pub is_action_camera: bool,

    #[bits(31)]
    _padding: u32,
}
