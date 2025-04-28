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

    /// Reserved for future use.
    _reserved1: u32,

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

// This only has a single bit, so maybe just using u32 is enough, since this is only read I assume
#[bitfield(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsActionCam {
    pub is_action_camera: bool,

    #[bits(31)]
    _padding: u32,
}
