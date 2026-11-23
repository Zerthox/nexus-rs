/// Specialization line of a build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Specialization {
    /// Specialization id, 0 if the line is empty.
    pub id: u32,

    /// Trait ids of the selected major traits in order adept, master, grandmaster.
    ///
    /// 0 if none is selected.
    pub traits: [u32; 3],
}

/// Build of the current character.
///
/// Payload of the [`RTAPI_BUILD_CHANGED`](super::event::RTAPI_BUILD_CHANGED) event.
/// Only valid for the duration of the event callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct BuildData {
    /// Size of the struct as raised by RTAPI.
    ///
    /// Fields are only ever appended, check this before reading fields added in later versions.
    pub size: u32,

    /// 0-based build template tab.
    pub template_index: u32,

    /// Profession 1-9.
    pub profession: u32,

    /// Selected specialization lines.
    pub specializations: [Specialization; 3],
}
