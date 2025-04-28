use super::RealTimeData;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameData {
    /// Game build number.
    pub game_build: u32,

    /// Current game state.
    pub game_state: Result<GameState, u32>,

    /// Language setting in game client.
    pub language: Result<GameLanguage, u32>,
}

impl GameData {
    pub unsafe fn read(data: *const RealTimeData) -> Self {
        Self {
            game_build: (*data).game_build,
            game_state: (*data).game_state.try_into(),
            language: (*data).language.try_into(),
        }
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive, IntoPrimitive,
)]
#[num_enum(error_type(name = u32, constructor = From::from))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
pub enum GameState {
    CharacterSelection,
    CharacterCreation,
    Cinematic,
    LoadingScreen,
    Gameplay,
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive, IntoPrimitive,
)]
#[num_enum(error_type(name = u32, constructor = From::from))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
pub enum GameLanguage {
    English,
    Korean,
    French,
    German,
    Spanish,
    Chinese,
}
