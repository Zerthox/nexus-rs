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

    /// Current level of the character
    pub character_level: u32,

    /// Current effective character level (Scaling)
    pub character_effective_level: u32,
}

impl PlayerData {
    /// Reads player data from the given data pointer.
    ///
    /// # Safety
    /// The pointer must be safe to read from.
    pub unsafe fn read(data: *const RealTimeData) -> Self {
        unsafe {
            Self {
                account_name: CStr::from_ptr((*data).account_name.as_ptr())
                    .to_string_lossy()
                    .into_owned(),
                character_name: CStr::from_ptr((*data).character_name.as_ptr())
                    .to_string_lossy()
                    .into_owned(),
                character_position: (*data).character_position,
                character_facing: (*data).character_facing,
                profession: (*data).profession,
                elite_specialization: (*data).elite_specialization,
                mount_index: (*data).mount_index,
                character_state: CharacterState::from_bits_retain((*data).character_state),
                character_level: (*data).character_level,
                character_effective_level: (*data).character_effective_level,
            }
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

/// Character attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Attributes {
    pub power: u32,
    pub precision: u32,
    pub toughness: u32,
    pub vitality: u32,
    pub ferocity: u32,
    pub healing_power: u32,
    pub condition_damage: u32,
    pub concentration: u32,
    pub expertise: u32,
    pub agony_resistance: u32,
}
