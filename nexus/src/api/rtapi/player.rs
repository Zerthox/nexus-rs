use super::RealTimeData;
use bitflags::bitflags;
use std::ffi::CStr;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerData {
    /// Account name of current player.
    pub account_name: String,

    /// Character name of current player.
    pub character_name: String,

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

    /// Current state of the character.
    pub character_state: CharacterState,
}

impl PlayerData {
    /// Reads player data from the given data pointer.
    ///
    /// # Safety
    /// The pointer must be safe to read from.
    pub unsafe fn read(data: *const RealTimeData) -> Self {
        Self {
            account_name: CStr::from_ptr((*data).account_name.as_ptr())
                .to_string_lossy()
                .into_owned(),
            character_name: CStr::from_ptr((*data).account_name.as_ptr())
                .to_string_lossy()
                .into_owned(),
            character_position: (*data).character_position,
            character_facing: (*data).character_facing,
            profession: (*data).profession,
            elite_specialization: (*data).elite_specialization,
            mount_index: (*data).mount_index,
            character_state: CharacterState::from_bits_retain((*data).character_state),
        }
    }
}

bitflags! {
    #[derive(
        Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
    )]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct CharacterState : u32 {
        /// Is alive.
        const IsAlive = 1 << 0;

        /// Is downed.
        const IsDowned = 1 << 1;

        /// Is in combat.
        const IsInCombat = 1 << 2;

        /// Is on water surface.
        const IsSwimming = 1 << 3;

        /// Is underwater.
        const IsUnderwater = 1 << 4;

        /// Is gliding.
        const IsGliding = 1 << 5;

        /// Is on flying mount.
        const IsFlying = 1 << 6;
    }
}
