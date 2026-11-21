use bitfields::bitfield;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::ffi::{CStr, c_char};

/// Combat agent.
///
/// Payload of the `RTAPI_COMBAT_AGENT_*` events.
/// Only valid for the duration of the event callback.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CombatAgent {
    /// Size of the struct as raised by RTAPI.
    ///
    /// Fields are only ever appended, check this before reading fields added in later versions.
    pub size: u32,

    /// Agent id, unique within the current map instance.
    pub id: u32,

    /// Agent type, see [`AgentType`].
    agent_type: u32,

    /// Species id for characters, gadget id for gadgets and attack targets.
    pub species_id: u32,

    /// Agent id of the top-most owner if the agent is a minion, otherwise 0.
    pub owner_id: u32,

    /// Attitude towards the player, see [`Attitude`].
    ///
    /// Characters only.
    attitude: u32,

    /// Profession 1-9 if the agent is a player, otherwise 0.
    pub profession: u32,

    /// Name in UTF-8.
    ///
    /// Empty until decoded and for hostile players.
    name: [c_char; 140],

    /// Account name in UTF-8.
    ///
    /// Only for players known to the player, for example party, squad or contacts.
    /// Never for hostile players.
    account_name: [c_char; 140],

    /// Agent flags.
    flags: CombatAgentFlags,
}

impl CombatAgent {
    /// Converts the agent to a [`CombatAgentOwned`].
    #[inline]
    pub fn to_owned(&self) -> CombatAgentOwned {
        self.into()
    }

    /// Returns the type of the agent.
    #[inline]
    pub fn agent_type(&self) -> Result<AgentType, u32> {
        self.agent_type.try_into()
    }

    /// Returns the attitude of the agent towards the player.
    #[inline]
    pub fn attitude(&self) -> Result<Attitude, u32> {
        self.attitude.try_into()
    }

    /// Returns the name of the agent as pointer.
    #[inline]
    pub const fn name_ptr(&self) -> *const c_char {
        self.name.as_ptr()
    }

    /// Returns the name of the agent as [`CStr`].
    #[inline]
    pub fn name_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }
    }

    /// Returns the name of the agent as owned [`String`].
    #[inline]
    pub fn name(&self) -> String {
        self.name_cstr().to_string_lossy().into_owned()
    }

    /// Returns the account name of the agent as pointer.
    #[inline]
    pub const fn account_name_ptr(&self) -> *const c_char {
        self.account_name.as_ptr()
    }

    /// Returns the account name of the agent as [`CStr`].
    #[inline]
    pub fn account_name_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.account_name.as_ptr()) }
    }

    /// Returns the account name of the agent as owned [`String`].
    #[inline]
    pub fn account_name(&self) -> String {
        self.account_name_cstr().to_string_lossy().into_owned()
    }

    /// Returns the flags of the agent.
    #[inline]
    pub const fn flags(&self) -> CombatAgentFlags {
        self.flags
    }

    /// Returns whether the agent is the player's own character.
    #[inline]
    pub const fn is_self(&self) -> bool {
        self.flags.is_self()
    }

    /// Returns whether the agent is a player.
    #[inline]
    pub const fn is_player(&self) -> bool {
        self.flags.is_player()
    }

    /// Returns whether the agent is owned by another agent, for example a pet, clone or summon.
    #[inline]
    pub const fn is_minion(&self) -> bool {
        self.flags.is_minion()
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CombatAgentFlags {
    pub is_self: bool,
    pub is_player: bool,
    pub is_minion: bool,

    #[bits(29)]
    _padding: u32,
}

/// Combat agent as owned version.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CombatAgentOwned {
    /// Agent id, unique within the current map instance.
    pub id: u32,

    /// Agent type.
    pub agent_type: Result<AgentType, u32>,

    /// Species id for characters, gadget id for gadgets and attack targets.
    pub species_id: u32,

    /// Agent id of the top-most owner if the agent is a minion, otherwise 0.
    pub owner_id: u32,

    /// Attitude towards the player. Characters only.
    pub attitude: Result<Attitude, u32>,

    /// Profession 1-9 if the agent is a player, otherwise 0.
    pub profession: u32,

    /// Name of the agent. Empty until decoded and for hostile players.
    pub name: String,

    /// Account name of the agent.
    ///
    /// Only for players known to the player, for example party, squad or contacts.
    pub account_name: String,

    /// Is the player's own character.
    pub is_self: bool,

    /// Is a player.
    pub is_player: bool,

    /// Is owned by another agent, for example a pet, clone or summon.
    pub is_minion: bool,
}

impl From<&CombatAgent> for CombatAgentOwned {
    fn from(agent: &CombatAgent) -> Self {
        Self {
            id: agent.id,
            agent_type: agent.agent_type(),
            species_id: agent.species_id,
            owner_id: agent.owner_id,
            attitude: agent.attitude(),
            profession: agent.profession,
            name: agent.name(),
            account_name: agent.account_name(),
            is_self: agent.is_self(),
            is_player: agent.is_player(),
            is_minion: agent.is_minion(),
        }
    }
}

/// Combat skill.
///
/// Payload of the `RTAPI_COMBAT_SKILL_*` events.
/// Only valid for the duration of the event callback.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CombatSkill {
    /// Size of the struct as raised by RTAPI.
    ///
    /// Fields are only ever appended, check this before reading fields added in later versions.
    pub size: u32,

    /// Skill id.
    pub id: u32,

    /// Name in UTF-8.
    ///
    /// Empty until decoded.
    name: [c_char; 140],
}

impl CombatSkill {
    /// Converts the skill to a [`CombatSkillOwned`].
    #[inline]
    pub fn to_owned(&self) -> CombatSkillOwned {
        self.into()
    }

    /// Returns the name of the skill as pointer.
    #[inline]
    pub const fn name_ptr(&self) -> *const c_char {
        self.name.as_ptr()
    }

    /// Returns the name of the skill as [`CStr`].
    #[inline]
    pub fn name_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }
    }

    /// Returns the name of the skill as owned [`String`].
    #[inline]
    pub fn name(&self) -> String {
        self.name_cstr().to_string_lossy().into_owned()
    }
}

/// Combat skill as owned version.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CombatSkillOwned {
    /// Skill id.
    pub id: u32,

    /// Name of the skill. Empty until decoded.
    pub name: String,
}

impl From<&CombatSkill> for CombatSkillOwned {
    fn from(skill: &CombatSkill) -> Self {
        Self {
            id: skill.id,
            name: skill.name(),
        }
    }
}

/// Combat event.
///
/// Every combat event payload starts with this.
/// Payload of the combat events without values.
/// Only contains events from the player's own combat log.
/// Only valid for the duration of the event callback.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombatEvent {
    /// Size of the full payload, for example the size of [`CombatHealthEvent`].
    ///
    /// Fields are only ever appended, check this before reading fields added in later versions.
    pub size: u32,

    /// Event type, see [`CombatEventType`].
    event_type: u32,

    /// Unix timestamp in milliseconds.
    pub time: u64,

    /// Source agent id, 0 if there is none.
    pub source_id: u32,

    /// Destination agent id, 0 if there is none.
    pub destination_id: u32,

    /// Skill id, 0 if there is none.
    pub skill_id: u32,

    /// Event flags.
    flags: CombatEventFlags,
}

impl CombatEvent {
    /// Returns the type of the event.
    #[inline]
    pub fn event_type(&self) -> Result<CombatEventType, u32> {
        self.event_type.try_into()
    }

    /// Returns the flags of the event.
    #[inline]
    pub const fn flags(&self) -> CombatEventFlags {
        self.flags
    }

    /// Returns whether the event is a bonus.
    #[inline]
    pub const fn is_bonus(&self) -> bool {
        self.flags.is_bonus()
    }

    /// Returns whether the event is a critical hit.
    #[inline]
    pub const fn is_critical(&self) -> bool {
        self.flags.is_critical()
    }

    /// Returns whether the event is a fumble.
    #[inline]
    pub const fn is_fumble(&self) -> bool {
        self.flags.is_fumble()
    }

    /// Returns whether the event is condition damage.
    #[inline]
    pub const fn is_condition_damage(&self) -> bool {
        self.flags.is_condition_damage()
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CombatEventFlags {
    pub is_bonus: bool,
    pub is_critical: bool,
    pub is_fumble: bool,
    pub is_condition_damage: bool,

    #[bits(28)]
    _padding: u32,
}

/// Payload of the `RTAPI_COMBAT_BREAKBAR` event.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombatBreakbarEvent {
    pub base: CombatEvent,

    /// Breakbar change in the game's breakbar points.
    pub points: f32,
}

/// Payload of the `RTAPI_COMBAT_ENERGY` event.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombatEnergyEvent {
    pub base: CombatEvent,
    pub energy: f32,
}

/// Payload of the `RTAPI_COMBAT_EXPERIENCE` event.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombatExperienceEvent {
    pub base: CombatEvent,
    pub experience: f32,
    pub bonus_experience: f32,
}

/// Payload of the `RTAPI_COMBAT_HEALTH` event.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombatHealthEvent {
    pub base: CombatEvent,

    /// Change in health. Negative is damage, positive is healing.
    pub health: f32,

    /// Change in barrier. Negative is damage, positive is barrier gained.
    pub barrier: f32,
}

/// Payload of the `RTAPI_COMBAT_KARMA` event.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombatKarmaEvent {
    pub base: CombatEvent,
    pub karma: f32,
}

/// Payload of the `RTAPI_COMBAT_LUCK` event.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombatLuckEvent {
    pub base: CombatEvent,
    pub luck: f32,
}

/// Payload of the `RTAPI_COMBAT_RESIST` event.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombatResistEvent {
    pub base: CombatEvent,

    /// Game id of the resisted effect, for example a condition or crowd control.
    pub effect: u32,
}

/// Payload of the `RTAPI_COMBAT_PVP_SCORE` and `RTAPI_COMBAT_WVW_SCORE` events.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombatScoreEvent {
    pub base: CombatEvent,
    pub score: i32,
}

/// Payload of the `RTAPI_COMBAT_WVW_EXPERIENCE` event.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombatWvWExperienceEvent {
    pub base: CombatEvent,
    pub experience: f32,
}

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
pub enum AgentType {
    Character,
    Gadget,
    AttackTarget,
}

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
pub enum Attitude {
    Friendly,
    Hostile,
    Indifferent,
    Neutral,
}

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
pub enum CombatEventType {
    Block,
    Breakbar,
    Combo,
    Death,
    Down,
    Energy,
    Evade,
    Experience,
    Health,
    Interrupt,
    InvulnerabilityAbsorb,
    Karma,
    Luck,
    MissedBySource,
    MissedTarget,
    PvPScore,
    Resist,
    SkillAbsorbed,
    SkillBegin,
    SkillEnd,
    SkillObstructed,
    SkillOutOfRange,
    WvWScoreRed,
    WvWScoreBlue,
    WvWScoreGreen,
    WvWExperience,
}
