use super::Event;
use std::ffi::c_void;

// TODO: optional typing with evtc crate

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
    pub event: *const c_void,
    pub src: *const c_void,
    pub dst: *const c_void,
    pub id: u64,
    pub rev: u64,
}
