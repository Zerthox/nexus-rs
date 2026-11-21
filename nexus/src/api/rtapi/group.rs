use super::RealTimeData;
use bitfields::bitfield;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::ffi::{CStr, c_char};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupData {
    /// Locations of squad markers in the game world as ingame coordinates.
    pub squad_markers: [[f32; 3]; 8],

    /// Type of current group.
    pub group_type: Result<GroupType, u32>,

    /// Number of members in current group.
    pub group_member_count: u32,
}

impl GroupData {
    /// Reads group data from the given data pointer.
    ///
    /// # Safety
    /// The pointer must be safe to read from.
    pub unsafe fn read(data: *const RealTimeData) -> Self {
        unsafe {
            Self {
                squad_markers: (*data).squad_markers,
                group_type: (*data).group_type.try_into(),
                group_member_count: (*data).group_member_count,
            }
        }
    }
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
pub enum GroupType {
    None,
    Party,
    RaidSquad,
    Squad,
}

/// Group member.
///
/// Payload of the [`RTAPI_GROUP_MEMBER_JOINED`](super::event::RTAPI_GROUP_MEMBER_JOINED),
/// [`RTAPI_GROUP_MEMBER_LEFT`](super::event::RTAPI_GROUP_MEMBER_LEFT)
/// and [`RTAPI_GROUP_MEMBER_UPDATE`](super::event::RTAPI_GROUP_MEMBER_UPDATE) events.
/// Only valid for the duration of the event callback.
///
/// This struct uses the C layout.
/// Instead of cloning this it is recommended to convert to [`GroupMemberOwned`] via [`Into`] or [`to_owned`](GroupMember::to_owned).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct GroupMember {
    /// Account name of the group member.
    account_name: [c_char; 140],

    /// Character name of the group member.
    character_name: [c_char; 140],

    /// Subgroup of the group member.
    ///
    /// 0 when in a party.
    /// 1-15 subgrups when in a squad.
    pub subgroup: u32,

    /// Profession of the group member.
    ///
    /// 0 when unknown, for example on loading screen or character select.
    pub profession: u32,

    /// 3rd specialization of the group member (not always elite).
    ///
    /// 0 when unknown, for example on loading screen or character select.
    pub elite_specialization: u32,

    /// Member flags.
    flags: GroupMemberFlags,

    /// Contact nickname of the player for this account in UTF-8, empty if none.
    nickname: [c_char; 140],
}

impl GroupMember {
    /// Converts the member to a [`GroupMemberOwned`].
    #[inline]
    pub fn to_owned(&self) -> GroupMemberOwned {
        self.into()
    }

    /// Returns the account name of the member as pointer.
    #[inline]
    pub const fn account_name_ptr(&self) -> *const c_char {
        self.account_name.as_ptr()
    }

    /// Returns the account name of the member as [`CStr`].
    #[inline]
    pub fn account_name_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.account_name.as_ptr()) }
    }

    /// Returns the account name of the member as owned [`String`].
    #[inline]
    pub fn account_name(&self) -> String {
        self.account_name_cstr().to_string_lossy().into_owned()
    }

    /// Returns the character name of the member as pointer.
    #[inline]
    pub const fn character_name_ptr(&self) -> *const c_char {
        self.character_name.as_ptr()
    }

    /// Returns the account name of the member as [`CStr`].
    #[inline]
    pub fn character_name_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.character_name.as_ptr()) }
    }

    /// Returns the character name of the member as owned [`String`].
    #[inline]
    pub fn character_name(&self) -> String {
        self.character_name_cstr().to_string_lossy().into_owned()
    }

    /// Returns the contact nickname of the member as pointer.
    #[inline]
    pub const fn nickname_ptr(&self) -> *const c_char {
        self.nickname.as_ptr()
    }

    /// Returns the contact nickname of the member as [`CStr`].
    ///
    /// Empty if the player has no nickname set for this account.
    #[inline]
    pub fn nickname_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.nickname.as_ptr()) }
    }

    /// Returns the contact nickname of the member as owned [`String`].
    ///
    /// Empty if the player has no nickname set for this account.
    #[inline]
    pub fn nickname(&self) -> String {
        self.nickname_cstr().to_string_lossy().into_owned()
    }

    /// Returns the flags of the member.
    #[inline]
    pub const fn flags(&self) -> GroupMemberFlags {
        self.flags
    }

    /// Returns whether the member is self (the local player).
    #[inline]
    pub const fn is_self(&self) -> bool {
        self.flags.is_self()
    }

    /// Returns whether the member is in the current instance.
    #[inline]
    pub const fn is_in_instance(&self) -> bool {
        self.flags.is_in_instance()
    }

    /// Returns whether the member if the commander of the current squad.
    #[inline]
    pub const fn is_commander(&self) -> bool {
        self.flags.is_commander()
    }

    /// Returns whether the member is a lieutenant in the current squad.
    #[inline]
    pub const fn is_lieutenant(&self) -> bool {
        self.flags.is_lieutenant()
    }

    /// Returns whether the member is alive.
    ///
    /// Only valid if the member is in the current instance.
    #[inline]
    pub const fn is_alive(&self) -> bool {
        self.flags.is_alive()
    }

    /// Returns whether the member is downed.
    ///
    /// Only valid if the member is in the current instance.
    #[inline]
    pub const fn is_downed(&self) -> bool {
        self.flags.is_downed()
    }

    /// Returns whether the member owns the map instance the group is in.
    #[inline]
    pub const fn is_instance_owner(&self) -> bool {
        self.flags.is_instance_owner()
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupMemberFlags {
    pub is_self: bool,
    pub is_in_instance: bool,
    pub is_commander: bool,
    pub is_lieutenant: bool,
    pub is_alive: bool,
    pub is_downed: bool,
    pub is_instance_owner: bool,

    #[bits(25)]
    _padding: u32,
}

/// Group Member as owned version.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupMemberOwned {
    /// Account name of the group member.
    pub account_name: String,

    /// Character name of the group member.
    pub character_name: String,

    /// 0 for parties, 1-15 according to the squad's subgroup
    pub subgroup: u32,

    /// 1-9 = Profession; 0 Unknown -> e.g. on loading screen or logged out
    pub profession: u32,

    /// Third Spec ID, not necessarily elite; or 0 Unknown -> e.g. on loading screen or logged out
    pub elite_specialization: u32,

    /// Is this member the player themselves?
    pub is_self: bool,

    /// Is in the same map instance as the player.
    pub is_in_instance: bool,

    /// Is this member a commander?
    pub is_commander: bool,

    /// Is this member a lieutenant?
    pub is_lieutenant: bool,

    /// Is alive. Only valid if in the same map instance as the player.
    pub is_alive: bool,

    /// Is downed. Only valid if in the same map instance as the player.
    pub is_downed: bool,

    /// Owns the map instance the group is in.
    pub is_instance_owner: bool,

    /// Contact nickname of the player for this account, empty if none.
    pub nickname: String,
}

impl From<&GroupMember> for GroupMemberOwned {
    fn from(member: &GroupMember) -> Self {
        Self {
            account_name: member.account_name(),
            character_name: member.character_name(),
            subgroup: member.subgroup,
            profession: member.profession,
            elite_specialization: member.elite_specialization,
            is_self: member.is_self(),
            is_in_instance: member.is_in_instance(),
            is_commander: member.is_commander(),
            is_lieutenant: member.is_lieutenant(),
            is_alive: member.is_alive(),
            is_downed: member.is_downed(),
            is_instance_owner: member.is_instance_owner(),
            nickname: member.nickname(),
        }
    }
}

/// Vitals of a group member.
///
/// Part of [`RealTimeGroupData`](super::data::RealTimeGroupData).
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GroupMemberVitals {
    /// Account name of the group member, matches [`GroupMember::account_name`].
    account_name: [c_char; 140],

    /// Health in percent, 0-100.
    ///
    /// Currently whole numbers, as the game shows them.
    /// 0 if the member is not in the current instance.
    pub health: f32,
}

impl GroupMemberVitals {
    /// Returns the account name of the member as pointer.
    #[inline]
    pub const fn account_name_ptr(&self) -> *const c_char {
        self.account_name.as_ptr()
    }

    /// Returns the account name of the member as [`CStr`].
    #[inline]
    pub fn account_name_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.account_name.as_ptr()) }
    }

    /// Returns the account name of the member as owned [`String`].
    #[inline]
    pub fn account_name(&self) -> String {
        self.account_name_cstr().to_string_lossy().into_owned()
    }
}
