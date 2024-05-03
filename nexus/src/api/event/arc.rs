//! [ArcDPS EVTC](https://deltaconnected.com/arcdps/) bridge events.

use super::Event;
use arcdps::evtc::{self, Agent};

/// ArcDPS EVTC combat local event.
pub const COMBAT_LOCAL: Event<CombatData> =
    unsafe { Event::new("EV_ARCDPS_COMBATEVENT_LOCAL_RAW") };

/// ArcDPS EVTC combat squad event.
pub const COMBAT_SQUAD: Event<CombatData> =
    unsafe { Event::new("EV_ARCDPS_COMBATEVENT_SQUAD_RAW") };

/// ArcDPS EVTC combat event data.
#[derive(Debug, Clone)]
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
