//! [ArcDPS EVTC](https://deltaconnected.com/arcdps/) bridge events.

use super::Event;
use arcdps::evtc::{self, Agent};
use std::ffi::{c_char, CStr};

/// ArcDPS EVTC combat local event.
pub const COMBAT_LOCAL: Event<CombatData> =
    unsafe { Event::new("EV_ARCDPS_COMBATEVENT_LOCAL_RAW") };

/// ArcDPS EVTC combat squad event.
pub const COMBAT_SQUAD: Event<CombatData> =
    unsafe { Event::new("EV_ARCDPS_COMBATEVENT_SQUAD_RAW") };

/// ArcDPS self join event.
///
/// Payload is [`AgentUpdate`] of the self player agent.
///
/// Events of this type are triggered upon map load.
/// The last event can be retriggered on demand by addons sending an [`REPLAY_SELF_JOIN`] event.
pub const SELF_JOIN: Event<AgentUpdate> = unsafe { Event::new("EV_ARCDPS_SELF_JOIN") };

/// Replays the last [`SELF_JOIN`] event.
pub const REPLAY_SELF_JOIN: Event<()> = unsafe { Event::new("EV_REPLAY_ARCDPS_SELF_JOIN") };

/// ArcDPS self leave event.
///
/// Payload is [`AgentUpdate`] of the self player agent.
///
/// Events of this type are triggered when changing instance or leaving a party / squad.
pub const SELF_LEAVE: Event<AgentUpdate> = unsafe { Event::new("EV_ARCDPS_SELF_LEAVE") };

/// ArcDPS squad join event.
///
/// Payload is [`AgentUpdate`] of an allied player agent.
/// Events of this type are triggered when allied players in your instance join your party/squad or when allied players in your party/squad join your instance.
/// These events have a 2 second delay.
///
/// Nexus tracks all players in your squad and can retrigger these events on demand by addons sending an [`REPLAY_SQUAD_JOIN`] event.
/// This is intended to be used during addon load, you should be careful to handle duplicates since this can be triggered by other addons.
pub const SQUAD_JOIN: Event<AgentUpdate> = unsafe { Event::new("EV_ARCDPS_SQUAD_JOIN") };

/// Replays [`SQUAD_JOIN`] events for the current squad.
pub const REPLAY_SQUAD_JOIN: Event<()> = unsafe { Event::new("EV_REPLAY_ARCDPS_SQUAD_JOIN") };

/// ArcDPS squad leave event.
///
/// Payload is [`AgentUpdate`] of an allied player agent.
///
/// Events of this type are triggered when allied players in your instance and party/squad either leave your instance or leave your party/squad.
/// You will not recieve these events if you are the one to change instance or leave the party/squad.
/// These events have a 2 second delay.
pub const SQUAD_LEAVE: Event<AgentUpdate> = unsafe { Event::new("EV_ARCDPS_SQUAD_LEAVE") };

/// ArcDPS target changed event.
///
/// Events of this type are triggered when you target an agent.
/// The last event can be retriggered on demand by addons sending an [`REPLAY_TARGET_CHAGNED`] event.
pub const TARGET_CHANGED: Event<AgentUpdate> = unsafe { Event::new("EV_ARCDPS_TARGET_CHANGED") };

/// Replays the [`TARGET_CHANGED`] event for the current target.
pub const REPLAY_TARGET_CHANGED: Event<()> =
    unsafe { Event::new("EV_REPLAY_ARCDPS_TARGET_CHANGED") };

/// ArcDPS player account name.
///
/// Triggered on first map load.
/// Can be triggered on demand by sending `"EV_REQUEST_ACCOUNT_NAME"`.
pub const ACCOUNT_NAME: Event<c_char> = unsafe { Event::new("EV_ACCOUNT_NAME") };

/// ArcDPS agent update.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct AgentUpdate {
    /// Account name.
    account: [c_char; 64],

    /// Character name.
    character: [c_char; 64],

    /// ArcDPS id of the agent.
    pub id: usize,

    /// Instance id of the agent.
    pub instance_id: usize,

    /// Whether the agent has been added or removed.
    added: u32,

    /// Whether the agent is the new target.
    target: u32,

    /// Whether the agent is self.
    is_self: u32,

    /// Agent profession.
    pub prof: u32,

    /// Agent elite specialization.
    pub elite: u32,

    /// Agent team.
    pub team: u16,

    /// Agent subgroup.
    pub subgroup: u16,
}

impl AgentUpdate {
    /// Returns the account name (if present).
    #[inline]
    pub fn account(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.account.as_ptr()) }
    }

    /// Returns the character name.
    #[inline]
    pub fn character(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.character.as_ptr()) }
    }

    /// Whether the agent has been added or removed.
    #[inline]
    pub fn is_added(&self) -> bool {
        self.added != 0
    }

    /// Whether the agent is the new target.
    #[inline]
    pub fn is_target(&self) -> bool {
        self.target != 0
    }

    /// Whether the agent is self.
    #[inline]
    pub fn is_self(&self) -> bool {
        self.is_self != 0
    }
}

/// ArcDPS EVTC combat event data.
#[derive(Debug)]
#[repr(C)]
pub struct CombatData {
    event: *const evtc::Event,
    src: *const Agent,
    dst: *const Agent,
    pub id: u64,
    pub rev: u64,
}

impl CombatData {
    #[inline]
    pub fn as_tuple(
        &self,
    ) -> (
        Option<&evtc::Event>,
        Option<&Agent>,
        Option<&Agent>,
        u64,
        u64,
    ) {
        (self.event(), self.src(), self.dst(), self.id, self.rev)
    }

    /// Returns a pointer to the [`Event`].
    #[inline]
    pub fn event_ptr(&self) -> *const evtc::Event {
        self.event
    }

    /// Returns the [`Event`].
    #[inline]
    pub fn event(&self) -> Option<&evtc::Event> {
        unsafe { self.event.as_ref() }
    }

    /// Returns a pointer to the source [`Agent`].
    #[inline]
    pub fn src_ptr(&self) -> *const Agent {
        self.src
    }

    /// Returns the [`Event`].
    #[inline]
    pub fn src(&self) -> Option<&Agent> {
        unsafe { self.src.as_ref() }
    }

    /// Returns a pointer to the destination [`Agent`].
    #[inline]
    pub fn dst_ptr(&self) -> *const Agent {
        self.dst
    }

    /// Returns the destination [`Agent`].
    #[inline]
    pub fn dst(&self) -> Option<&Agent> {
        unsafe { self.dst.as_ref() }
    }
}
