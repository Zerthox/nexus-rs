use crate::on_unload;

/// A revertible action.
///
/// Returned by several functions registering callbacks that may have to be unregistered later.
/// This type is marked as `must_use` to force being explicit about handling reversions.
///
/// There several ways to deal with a received [`Revertible`]:
/// - Call [`revert_on_unload`](Revertible::revert_on_unload) to automatically revert on addon unload.
/// - Call [`leak`](Revertible::leak) or drop it (explicitly or implicitly) to discard and unregister manually.
/// - Keep the [`Revertible`] and call [`revert`](Revertible::revert) later.
/// - Turn it into a callable via [`into_inner`](Revertible::into_inner) and call it later.
#[must_use]
#[derive(Debug)]
#[repr(transparent)]
pub struct Revertible<F>
where
    F: FnOnce() + Send + 'static,
{
    revert: F,
}

impl<F> Revertible<F>
where
    F: FnOnce() + Send + 'static,
{
    /// Creates a new revertible action.
    #[inline]
    pub const fn new(revert: F) -> Self {
        Self { revert }
    }

    /// Turns the revertible into the inner callable.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> F {
        self.revert
    }

    /// Leaves the action to not be reverted.
    ///
    /// The reversion has to be performed manually.
    /// Effectively the same as dropping the revertible.
    #[inline]
    pub fn leak(self) {}

    /// Reverts the action.
    #[inline]
    pub fn revert(self) {
        (self.revert)()
    }

    /// Submits the revertible to be reverted on unload.
    #[inline]
    pub fn revert_on_unload(self) {
        on_unload(self.into_inner())
    }
}

impl<F> From<F> for Revertible<F>
where
    F: FnOnce() + Send + 'static,
{
    #[inline]
    fn from(revert: F) -> Self {
        Self::new(revert)
    }
}
