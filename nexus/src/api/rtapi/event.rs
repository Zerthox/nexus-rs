use super::{
    CombatAgent, CombatBreakbarEvent, CombatEnergyEvent, CombatEvent, CombatExperienceEvent,
    CombatHealthEvent, CombatKarmaEvent, CombatLuckEvent, CombatResistEvent, CombatScoreEvent,
    CombatSkill, CombatWvWExperienceEvent, GroupMember,
};
use crate::event::Event;

/// RealTime API group member joined event.
pub const RTAPI_GROUP_MEMBER_JOINED: Event<GroupMember> =
    unsafe { Event::new("RTAPI_GROUP_MEMBER_JOINED") };

/// RealTime API group member left event.
pub const RTAPI_GROUP_MEMBER_LEFT: Event<GroupMember> =
    unsafe { Event::new("RTAPI_GROUP_MEMBER_LEFT") };

/// RealTime API group member updated event.
pub const RTAPI_GROUP_MEMBER_UPDATE: Event<GroupMember> =
    unsafe { Event::new("RTAPI_GROUP_MEMBER_UPDATED") };

/// RealTime API combat agent added event.
///
/// Raised when an agent is seen for the first time, before any event that references it.
pub const RTAPI_COMBAT_AGENT_ADDED: Event<CombatAgent> =
    unsafe { Event::new("RTAPI_COMBAT_AGENT_ADDED") };

/// RealTime API combat agent updated event.
///
/// Raised when agent data changes, for example when its name was decoded.
pub const RTAPI_COMBAT_AGENT_UPDATED: Event<CombatAgent> =
    unsafe { Event::new("RTAPI_COMBAT_AGENT_UPDATED") };

/// RealTime API combat agent removed event.
///
/// Raised when the agent left the combat log or the map changed. Its id may be reused after.
pub const RTAPI_COMBAT_AGENT_REMOVED: Event<CombatAgent> =
    unsafe { Event::new("RTAPI_COMBAT_AGENT_REMOVED") };

/// RealTime API combat skill added event.
///
/// Raised when a skill is seen for the first time, before any event that references it.
pub const RTAPI_COMBAT_SKILL_ADDED: Event<CombatSkill> =
    unsafe { Event::new("RTAPI_COMBAT_SKILL_ADDED") };

/// RealTime API combat skill updated event.
///
/// Raised when skill data changes, for example when its name was decoded.
pub const RTAPI_COMBAT_SKILL_UPDATED: Event<CombatSkill> =
    unsafe { Event::new("RTAPI_COMBAT_SKILL_UPDATED") };

/// RealTime API combat block event.
pub const RTAPI_COMBAT_BLOCK: Event<CombatEvent> = unsafe { Event::new("RTAPI_COMBAT_BLOCK") };

/// RealTime API combat breakbar event.
pub const RTAPI_COMBAT_BREAKBAR: Event<CombatBreakbarEvent> =
    unsafe { Event::new("RTAPI_COMBAT_BREAKBAR") };

/// RealTime API combat combo event.
pub const RTAPI_COMBAT_COMBO: Event<CombatEvent> = unsafe { Event::new("RTAPI_COMBAT_COMBO") };

/// RealTime API combat death event.
pub const RTAPI_COMBAT_DEATH: Event<CombatEvent> = unsafe { Event::new("RTAPI_COMBAT_DEATH") };

/// RealTime API combat down event.
pub const RTAPI_COMBAT_DOWN: Event<CombatEvent> = unsafe { Event::new("RTAPI_COMBAT_DOWN") };

/// RealTime API combat energy event.
pub const RTAPI_COMBAT_ENERGY: Event<CombatEnergyEvent> =
    unsafe { Event::new("RTAPI_COMBAT_ENERGY") };

/// RealTime API combat evade event.
pub const RTAPI_COMBAT_EVADE: Event<CombatEvent> = unsafe { Event::new("RTAPI_COMBAT_EVADE") };

/// RealTime API combat experience event.
pub const RTAPI_COMBAT_EXPERIENCE: Event<CombatExperienceEvent> =
    unsafe { Event::new("RTAPI_COMBAT_EXPERIENCE") };

/// RealTime API combat health event.
pub const RTAPI_COMBAT_HEALTH: Event<CombatHealthEvent> =
    unsafe { Event::new("RTAPI_COMBAT_HEALTH") };

/// RealTime API combat interrupt event.
pub const RTAPI_COMBAT_INTERRUPT: Event<CombatEvent> =
    unsafe { Event::new("RTAPI_COMBAT_INTERRUPT") };

/// RealTime API combat invulnerability absorb event.
pub const RTAPI_COMBAT_INVULNERABILITY_ABSORB: Event<CombatEvent> =
    unsafe { Event::new("RTAPI_COMBAT_INVULNERABILITY_ABSORB") };

/// RealTime API combat karma event.
pub const RTAPI_COMBAT_KARMA: Event<CombatKarmaEvent> = unsafe { Event::new("RTAPI_COMBAT_KARMA") };

/// RealTime API combat luck event.
pub const RTAPI_COMBAT_LUCK: Event<CombatLuckEvent> = unsafe { Event::new("RTAPI_COMBAT_LUCK") };

/// RealTime API combat missed by source event.
pub const RTAPI_COMBAT_MISSED_BY_SOURCE: Event<CombatEvent> =
    unsafe { Event::new("RTAPI_COMBAT_MISSED_BY_SOURCE") };

/// RealTime API combat missed target event.
pub const RTAPI_COMBAT_MISSED_TARGET: Event<CombatEvent> =
    unsafe { Event::new("RTAPI_COMBAT_MISSED_TARGET") };

/// RealTime API combat PvP score event.
pub const RTAPI_COMBAT_PVP_SCORE: Event<CombatScoreEvent> =
    unsafe { Event::new("RTAPI_COMBAT_PVP_SCORE") };

/// RealTime API combat resist event.
pub const RTAPI_COMBAT_RESIST: Event<CombatResistEvent> =
    unsafe { Event::new("RTAPI_COMBAT_RESIST") };

/// RealTime API combat skill absorbed event.
pub const RTAPI_COMBAT_SKILL_ABSORBED: Event<CombatEvent> =
    unsafe { Event::new("RTAPI_COMBAT_SKILL_ABSORBED") };

/// RealTime API combat skill begin event.
pub const RTAPI_COMBAT_SKILL_BEGIN: Event<CombatEvent> =
    unsafe { Event::new("RTAPI_COMBAT_SKILL_BEGIN") };

/// RealTime API combat skill end event.
pub const RTAPI_COMBAT_SKILL_END: Event<CombatEvent> =
    unsafe { Event::new("RTAPI_COMBAT_SKILL_END") };

/// RealTime API combat skill obstructed event.
pub const RTAPI_COMBAT_SKILL_OBSTRUCTED: Event<CombatEvent> =
    unsafe { Event::new("RTAPI_COMBAT_SKILL_OBSTRUCTED") };

/// RealTime API combat skill out of range event.
pub const RTAPI_COMBAT_SKILL_OUT_OF_RANGE: Event<CombatEvent> =
    unsafe { Event::new("RTAPI_COMBAT_SKILL_OUT_OF_RANGE") };

/// RealTime API combat WvW score event.
///
/// The event type is one of `WvWScoreRed`, `WvWScoreBlue` or `WvWScoreGreen`.
pub const RTAPI_COMBAT_WVW_SCORE: Event<CombatScoreEvent> =
    unsafe { Event::new("RTAPI_COMBAT_WVW_SCORE") };

/// RealTime API combat WvW experience event.
pub const RTAPI_COMBAT_WVW_EXPERIENCE: Event<CombatWvWExperienceEvent> =
    unsafe { Event::new("RTAPI_COMBAT_WVW_EXPERIENCE") };
