use std::ffi::{CStr, c_char};

/// Guild of the account.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Guild {
    /// GUID as the official API writes it, for example `4BBB52AA-D768-4FC6-8EDE-C299F2822F0F`.
    id: [c_char; 40],

    /// Name in UTF-8.
    ///
    /// Only known for the represented guild, otherwise empty.
    name: [c_char; 140],

    /// Tag in UTF-8, without brackets.
    ///
    /// Not provided yet, always empty.
    tag: [c_char; 8],
}

impl Guild {
    /// Returns the guild id as [`CStr`].
    #[inline]
    pub fn id_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.id.as_ptr()) }
    }

    /// Returns the guild id as owned [`String`].
    #[inline]
    pub fn id(&self) -> String {
        self.id_cstr().to_string_lossy().into_owned()
    }

    /// Returns the guild name as [`CStr`].
    ///
    /// Only known for the represented guild, otherwise empty.
    /// Look up other guilds via the official API endpoint `/v2/guild/:id`.
    #[inline]
    pub fn name_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }
    }

    /// Returns the guild name as owned [`String`].
    ///
    /// Only known for the represented guild, otherwise empty.
    /// Look up other guilds via the official API endpoint `/v2/guild/:id`.
    #[inline]
    pub fn name(&self) -> String {
        self.name_cstr().to_string_lossy().into_owned()
    }

    /// Returns the guild tag as [`CStr`].
    ///
    /// Not provided yet, always empty.
    #[inline]
    pub fn tag_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.tag.as_ptr()) }
    }

    /// Returns the guild tag as owned [`String`].
    ///
    /// Not provided yet, always empty.
    #[inline]
    pub fn tag(&self) -> String {
        self.tag_cstr().to_string_lossy().into_owned()
    }
}

/// Guilds of the account.
///
/// Payload of the `RTAPI_GUILDS_CHANGED` event.
/// The payload has variable length, use [`guilds`](GuildList::guilds) to access the entries.
/// Only valid for the duration of the event callback.
#[derive(Debug)]
#[repr(C)]
pub struct GuildList {
    /// Size of the whole payload in bytes, including all entries.
    pub size: u32,

    /// Number of guilds, may be 0.
    pub count: u32,

    /// First entry, the payload holds [`count`](GuildList::count) of them.
    guilds: [Guild; 1],
}

impl GuildList {
    /// Returns the guilds of the account.
    ///
    /// Only call this on the list passed to the event callback.
    /// A copy of the struct holds only the first entry.
    #[inline]
    pub fn guilds(&self) -> &[Guild] {
        unsafe { std::slice::from_raw_parts(self.guilds.as_ptr(), self.count as usize) }
    }
}
