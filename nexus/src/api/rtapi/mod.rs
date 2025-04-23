pub mod raw;

// TODO: impl From for references to reduce memcpy

use std::ffi::CStr;

// TODO: enums for professions and elite_specs?
//
/// Group Member
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct GroupMember {
    /// Account name of the group member
    pub account_name: String,
    /// Character name of the group member
    pub character_name: String,
    /// 0 for parties, 1-15 according to the squad's subgroup
    pub subgroup: i32,
    /// 0-9 = Profession; -1 Unknown -> e.g. on loading screen or logged out
    pub profession: i32,
    /// Third Spec ID, not necessarily elite; or -1 Unknown -> e.g. on loading screen or logged out
    pub elite_specialization: i32,
    /// Is this member the player themselves?
    pub is_self: bool,
    /// Is in the same map instance as the player.
    pub is_in_instance: bool,
    /// Is this member a commander?
    pub is_commander: bool,
    /// Is this member a lieutenant?
    pub is_lieutenant: bool,
}

impl raw::GroupMember {
    pub fn to_owned(self) -> GroupMember {
        self.into()
    }
}

impl From<raw::GroupMember> for GroupMember {
    fn from(raw: raw::GroupMember) -> Self {
        Self {
            account_name: unsafe { CStr::from_ptr(raw.account_name.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
            character_name: unsafe { CStr::from_ptr(raw.character_name.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
            subgroup: raw.subgroup,
            profession: raw.profession,
            elite_specialization: raw.elite_specialization,
            is_self: raw.flags.is_self(),
            is_in_instance: raw.flags.is_in_instance(),
            is_commander: raw.flags.is_commander(),
            is_lieutenant: raw.flags.is_lieutenant(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct GameData {
    /// Game builld number
    pub game_build: i32,
    /// Current game state
    pub game_state: raw::GameState,
    /// Language setting
    pub language: raw::GameLanguage,
}

impl From<raw::RealTimeData> for GameData {
    fn from(raw: raw::RealTimeData) -> Self {
        Self {
            game_build: raw.game_build,
            game_state: raw.game_state,
            language: raw.language,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct InstanceData {
    /// Time of day
    pub time_of_day: raw::TimeOfDay,
    /// Current map ID
    pub map_id: i32,
    /// Type of the map
    pub map_type: raw::MapType,
    /// IP address (4 bytes)
    pub ip_address: std::net::Ipv4Addr,
    /// Location of cursor in the world
    pub cursor: [f32; 3],
    /// Locations of squad markers
    /// Index is type of marker
    pub squad_markers: [[f32; 3]; 8],
    /// Type of group
    pub group_type: raw::GroupType,
    /// Number of group members
    pub group_member_count: u32,
}

impl From<raw::RealTimeData> for InstanceData {
    fn from(raw: raw::RealTimeData) -> Self {
        Self {
            time_of_day: raw.time_of_day,
            map_id: raw.map_id,
            map_type: raw.map_type,
            ip_address: raw.ip_address.into(),
            cursor: raw.cursor,
            squad_markers: raw.squad_markers,
            group_type: raw.group_type,
            group_member_count: raw.group_member_count,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PlayerData {
    /// Player's account name
    pub account_name: String,
    /// Player's character name
    pub character_name: String,
    /// Player's character position
    pub character_position: [f32; 3],
    /// Player's character facing direction
    pub character_facing: [f32; 3],
    /// Player's profession
    pub profession: i32,
    /// Player's elite specialization
    pub elite_specialization: i32,
    /// Index of the mount, if applicable
    pub mount_index: i32,
    /// Current state of the character
    pub character_state: raw::CharacterState,
}

impl From<raw::RealTimeData> for PlayerData {
    fn from(raw: raw::RealTimeData) -> Self {
        Self {
            account_name: unsafe { CStr::from_ptr(raw.account_name.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
            character_name: unsafe { CStr::from_ptr(raw.character_name.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
            character_position: raw.character_position,
            character_facing: raw.character_facing,
            profession: raw.profession,
            elite_specialization: raw.elite_specialization,
            mount_index: raw.mount_index,
            character_state: raw.character_state,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Camera {
    /// Camera position in the world
    pub position: [f32; 3],
    /// Camera facing direction
    pub facing: [f32; 3],
    /// Camera field of view
    pub fov: f32,
    /// Is this an action camera?
    pub is_action_camera: bool,
}

impl From<raw::RealTimeData> for Camera {
    fn from(raw: raw::RealTimeData) -> Self {
        Self {
            position: raw.camera_position,
            facing: raw.camera_facing,
            fov: raw.camera_fov,
            is_action_camera: raw.is_action_camera.is_action_camera(),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RealTimeData {
    /// Game Data
    pub game_data: GameData,
    /// Instance/World Data
    pub instance: InstanceData,
    /// Player Data
    pub player: PlayerData,
    /// Camera Data
    pub camera: Camera,
}

impl From<raw::RealTimeData> for RealTimeData {
    fn from(raw: raw::RealTimeData) -> Self {
        Self {
            game_data: raw.into(),
            instance: raw.into(),
            player: raw.into(),
            camera: raw.into(),
        }
    }
}

impl raw::RealTimeData {
    pub fn to_owned(self) -> RealTimeData {
        self.into()
    }
}
