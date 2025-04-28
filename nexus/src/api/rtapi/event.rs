use super::GroupMember;
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
