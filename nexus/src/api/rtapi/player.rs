use super::{GuildOwned, RealTimeData};
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

    /// Guild the character represents, [`None`] if not representing a guild.
    ///
    /// The tag is always empty, RTAPI does not provide it here.
    pub represented_guild: Option<GuildOwned>,
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
                build_template_index: (*data).build_template_index,
                gear_template_index: (*data).gear_template_index,
                base_attributes: (*data).base_attributes,
                effective_attributes: (*data).effective_attributes,
                represented_guild: {
                    let id = CStr::from_ptr((*data).represented_guild_id.as_ptr());
                    if id.is_empty() {
                        None
                    } else {
                        Some(GuildOwned {
                            id: id.to_string_lossy().into_owned(),
                            name: CStr::from_ptr((*data).represented_guild_id.as_ptr())
                                .to_string_lossy()
                                .into_owned(),
                            tag: String::new(),
                        })
                    }
                },
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
