//! FFI helpers.

#![allow(clippy::missing_safety_doc)]

mod path;
mod string;

pub use self::{path::*, string::*};

use std::ptr;

/// Helper trait to handle `Option<&T>`.
pub trait OptionRefExt<T> {
    /// Returns the string as [`T`] pointer or `null`.
    fn as_ptr_opt(&self) -> *const T;
}

impl<T> OptionRefExt<T> for Option<&T> {
    #[inline]
    fn as_ptr_opt(&self) -> *const T {
        self.map(|string| string as *const _).unwrap_or(ptr::null())
    }
}
