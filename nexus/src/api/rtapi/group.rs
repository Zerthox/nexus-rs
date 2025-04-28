use super::RealTimeData;
use bitfields::bitfield;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::ffi::{c_char, CStr};

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
        Self {
            squad_markers: (*data).squad_markers,
            group_type: (*data).group_type.try_into(),
            group_member_count: (*data).group_member_count,
        }
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive, IntoPrimitive,
)]
#[num_enum(error_type(name = u32, constructor = From::from))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
pub enum GroupType {
    None,
    Party,
    RaidSquad,
    Squad,
}

/// Group member.
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
}

#[bitfield(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupMemberFlags {
    pub is_self: bool,
    pub is_in_instance: bool,
    pub is_commander: bool,
    pub is_lieutenant: bool,

    #[bits(28)]
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

    /// 0-9 = Profession; -1 Unknown -> e.g. on loading screen or logged out
    pub profession: u32,

    /// Third Spec ID, not necessarily elite; or -1 Unknown -> e.g. on loading screen or logged out
    pub elite_specialization: u32,

    /// Is this member the player themselves?
    pub is_self: bool,

    /// Is in the same map instance as the player.
    pub is_in_instance: bool,

    /// Is this member a commander?
    pub is_commander: bool,

    /// Is this member a lieutenant?
    pub is_lieutenant: bool,
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
        }
    }
}
