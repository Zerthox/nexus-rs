//! [RTAPI](https://github.com/RaidcoreGG/GW2-RealTime-API-Releases) events.

use super::Event;
use crate::rtapi::raw::{
    GroupMember, EV_RTAPI_GROUP_MEMBER_JOINED, EV_RTAPI_GROUP_MEMBER_LEFT,
    EV_RTAPI_GROUP_MEMBER_UPDATED,
};

/// RTAPI group member joined event.
pub const RTAPI_GROUP_MEMBER_JOINED: Event<GroupMember> =
    unsafe { Event::new(EV_RTAPI_GROUP_MEMBER_JOINED) };

/// RTAPI group member left event.
pub const RTAPI_GROUP_MEMBER_LEFT: Event<GroupMember> =
    unsafe { Event::new(EV_RTAPI_GROUP_MEMBER_LEFT) };

/// RTAPI group member updated event.
pub const RTAPI_GROUP_MEMBER_UPDATE: Event<GroupMember> =
    unsafe { Event::new(EV_RTAPI_GROUP_MEMBER_UPDATED) };
