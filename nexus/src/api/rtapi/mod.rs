//! Bindings for Nexus RealTime API.

mod camera;
pub mod data;
pub mod event;
mod game;
mod group;
mod player;
mod world;

pub use self::{camera::*, game::*, group::*, player::*, world::*};

use self::data::RealTimeData;
use std::ptr::NonNull;

/// Interface for RealTime API data link.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct RealTimePtr(NonNull<RealTimeData>);

impl RealTimePtr {
    /// Signature of the RealTime API addon.
    pub const SIG: i32 = RealTimeData::SIG;

    /// Retrieves the interface for the data link.
    #[inline]
    pub fn get() -> Option<Self> {
        NonNull::new(RealTimeData::get_ptr().cast_mut()).map(Self)
    }

    /// Returns a raw pointer to the [`RealTimeData`].
    #[inline]
    pub const fn as_ptr(&self) -> *const RealTimeData {
        self.0.as_ptr()
    }

    /// Returns a [`NonNull`] to the [`RealTimeData`].
    #[inline]
    pub const fn as_non_null(&self) -> NonNull<RealTimeData> {
        self.0
    }

    /// Returns a reference to the value.
    ///
    /// This is dangerous since the memory is volatile and holding a reference can easily violate aliasing rules.
    ///
    /// # Safety
    /// See [NonNull::as_ref].
    #[inline]
    pub unsafe fn as_ref(&self) -> &RealTimeData {
        self.0.as_ref()
    }

    /// Checks whether the link is active.
    #[inline]
    pub fn is_active(&self) -> bool {
        unsafe { (*self.as_ptr()).game_build != 0 }
    }

    /// Reads the [`GameData`].
    #[inline]
    pub fn read_game(&self) -> Option<GameData> {
        self.is_active()
            .then(|| unsafe { GameData::read(self.as_ptr()) })
    }

    /// Reads the [`WorldData`].
    #[inline]
    pub fn read_world(&self) -> Option<WorldData> {
        self.is_active()
            .then(|| unsafe { WorldData::read(self.as_ptr()) })
    }

    /// Reads the [`GroupData`].
    #[inline]
    pub fn read_group(&self) -> Option<GroupData> {
        self.is_active()
            .then(|| unsafe { GroupData::read(self.as_ptr()) })
    }

    /// Reads the [`PlayerData`].
    #[inline]
    pub fn read_player(&self) -> Option<PlayerData> {
        self.is_active()
            .then(|| unsafe { PlayerData::read(self.as_ptr()) })
    }

    /// Reads the [`CameraData`].
    #[inline]
    pub fn read_camera(&self) -> Option<CameraData> {
        self.is_active()
            .then(|| unsafe { CameraData::read(self.as_ptr()) })
    }
}
