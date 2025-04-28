use super::RealTimeData;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::net::Ipv4Addr;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct WorldData {
    /// Tyrian time of day.
    pub time_of_day: Result<TimeOfDay, u32>,

    /// Map id of current map.
    pub map_id: u32,

    /// Map type of current map.
    pub map_type: Result<MapType, u32>,

    /// IP address of current server.
    pub ip_address: Ipv4Addr,

    /// Location of cursor in the game world as ingame coordinates.
    pub cursor: [f32; 3],
}

impl WorldData {
    pub unsafe fn read(data: *const RealTimeData) -> Self {
        Self {
            time_of_day: (*data).time_of_day.try_into(),
            map_id: (*data).map_id,
            map_type: (*data).map_type.try_into(),
            ip_address: (*data).ip_address.into(),
            cursor: (*data).cursor,
        }
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive, IntoPrimitive,
)]
#[num_enum(error_type(name = u32, constructor = From::from))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
pub enum TimeOfDay {
    Dawn,
    Day,
    Dusk,
    Night,
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive, IntoPrimitive,
)]
#[num_enum(error_type(name = u32, constructor = From::from))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
pub enum MapType {
    AutoRedirect,
    CharacterCreation,
    PvP,
    GvG,
    Instance,
    Public,
    Tournament,
    Tutorial,
    UserTournament,
    WvWEternalBattlegrounds,
    WvWBlueBorderlands,
    WvWGreenBorderlands,
    WvWRedBorderlands,
    WVWFortunesVale,
    WvWObsidianSanctum,
    WvWEdgeOfTheMists,
    PublicMini,
    BigBattle,
    WvWLounge,
}
