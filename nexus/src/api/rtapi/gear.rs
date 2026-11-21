use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Equipment slot, used as index into [`GearData::slots`].
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive, IntoPrimitive,
)]
#[num_enum(error_type(name = u32, constructor = From::from))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIter,
        strum::IntoStaticStr,
        strum::VariantArray,
        strum::VariantNames
    )
)]
#[repr(u32)]
pub enum GearSlotType {
    Helm,
    Shoulders,
    Coat,
    Gloves,
    Leggings,
    Boots,
    Back,
    Amulet,
    Accessory1,
    Accessory2,
    Ring1,
    Ring2,
    WeaponA1,
    WeaponA2,
    WeaponB1,
    WeaponB2,
    AquaticHelm,
    AquaticWeaponA,
    AquaticWeaponB,
    Relic,
}

/// Item equipped in a gear slot.
///
/// Ids match the official API endpoints `/v2/items` and `/v2/itemstats`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct GearSlot {
    /// Item id, 0 if the slot is empty.
    pub item_id: u32,

    /// Itemstat id, 0 if none.
    pub stat_id: u32,

    /// Rune or sigil item ids, 0 if empty.
    pub upgrades: [u32; 2],

    /// Infusion or enrichment item ids, 0 if empty.
    pub infusions: [u32; 3],
}

impl GearSlot {
    /// Returns whether the slot is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.item_id == 0
    }
}

/// Equipment of the current character.
///
/// Payload of the `RTAPI_GEAR_CHANGED` event.
/// Always the PvE equipment, also in PvP.
/// Only valid for the duration of the event callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct GearData {
    /// Size of the struct as raised by RTAPI.
    ///
    /// Fields are only ever appended, check this before reading fields added in later versions.
    pub size: u32,

    /// 0-based equipment template tab.
    pub template_index: u32,

    /// Equipped items, indexed by [`GearSlotType`].
    pub slots: [GearSlot; GearData::SLOT_COUNT],
}

// no std::mem::variant_count in stable
const _: () = assert!(GearSlotType::Relic as usize == GearData::SLOT_COUNT - 1);
impl GearData {
    /// Number of gear slots.
    pub const SLOT_COUNT: usize = 20;

    /// Returns the item in the given slot.
    #[inline]
    pub const fn slot(&self, slot: GearSlotType) -> &GearSlot {
        &self.slots[slot as usize]
    }
}
