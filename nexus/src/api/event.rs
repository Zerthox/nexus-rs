use crate::{addon_api, revertible::Revertible, util::str_to_c, AddonApi};
use std::{
    ffi::{c_char, c_void},
    mem,
};

pub type RawEventConsume<T> = extern "C-unwind" fn(event_args: *const T);

pub type RawEventConsumeUnknown = RawEventConsume<c_void>;

pub type RawEventRaise =
    unsafe extern "C-unwind" fn(identifier: *const c_char, event_data: *const c_void);

pub type RawEventRaiseNotification = unsafe extern "C-unwind" fn(identifier: *const c_char);

pub type RawEventSubscribe = unsafe extern "C-unwind" fn(
    identifier: *const c_char,
    consume_callback: RawEventConsumeUnknown,
);

/// Mumble identity update event identifier.
pub const MUMBLE_IDENTITY_UPDATED: &str = "EV_MUMBLE_IDENTITY_UPDATED";

/// Window resized event identifier.
pub const WINDOW_RESIZED: &str = "EV_WINDOW_RESIZED";

/// Subscribes to an event with a raw callback using an unknown payload.
///
/// Returns a [`Revertible`] to revert the subscribe.
pub fn event_subscribe_unknown(
    identifier: impl AsRef<str>,
    callback: RawEventConsumeUnknown,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let identifier = str_to_c(identifier, "failed to convert event identifier");
    let AddonApi {
        event_subscribe,
        event_unsubscribe,
        ..
    } = addon_api();
    unsafe { event_subscribe(identifier.as_ptr(), callback) };
    let revert = move || unsafe { event_unsubscribe(identifier.as_ptr(), callback) };
    revert.into()
}

/// Subscribes to an event with a raw callback using a typed payload.
///
/// Returns a [`Revertible`] to revert the subscribe.
///
/// # Safety
/// The caller must guarantee that the passed event identifier comes with valid data of the given type.
pub unsafe fn event_subscribe_typed<T>(
    identifier: impl AsRef<str>,
    callback: RawEventConsume<T>,
) -> Revertible<impl Fn() + Send + Sync + Clone + 'static> {
    let identifier = str_to_c(identifier, "failed to convert event identifier");
    let AddonApi {
        event_subscribe,
        event_unsubscribe,
        ..
    } = addon_api();
    let callback = unsafe { mem::transmute(callback) };
    unsafe { event_subscribe(identifier.as_ptr(), callback) };
    let revert = move || unsafe { event_unsubscribe(identifier.as_ptr(), callback) };
    revert.into()
}

/// Unsubscribes a previously registered raw event callback.
pub fn event_unsubscribe(identifier: impl AsRef<str>, callback: RawEventConsumeUnknown) {
    let identifier = str_to_c(identifier, "failed to convert event identifier");
    let AddonApi {
        event_unsubscribe, ..
    } = addon_api();
    unsafe { event_unsubscribe(identifier.as_ptr(), callback) }
}

/// Macro to subscribe to an event with a wrapped callback.
///
/// This macro is [unsafe](https://doc.rust-lang.org/std/keyword.unsafe.html).
/// See [`event_subscribe_typed`] for more information.
///
/// Returns a [`Revertible`] to revert the subscribe.
///
/// # Usage
/// ```no_run
/// # use nexus::event::*;
/// unsafe {
///     event_subscribe!("MY_EVENT" => i32, |args| {
///         println!("Received {args:?}");
///     })
/// }.revert_on_unload();
/// ```
///
/// The event identifier may be dynamic and the callback may be a function name.
/// ```no_run
/// # use nexus::event::*;
/// let event: &str = "MY_EVENT";
/// fn event_callback(event_args: Option<&i32>) {
///     println!("Received {event_args:?}");
/// }
/// let revertible = unsafe { event_subscribe!(event => i32, event_callback) };
/// revertible.revert();
/// ```
///
/// The `unsafe` keyword may be moved into the macro call:
/// ```no_run
/// # use nexus::event::*;
/// # fn event_callback(_: Option<&()>) {}
/// event_subscribe!(unsafe "MY_EVENT" => (), event_callback);
/// ```
///
/// # Safety
/// See [`event_subscribe_typed`].
#[macro_export]
macro_rules! event_subscribe {
    ( unsafe $event:expr , $ty:ty , $callback:expr $(,)? ) => {
        unsafe { $crate::event::event_subscribe!($event => $ty, $callback) }
    };
    ( unsafe $event:expr => $ty:ty , $callback:expr $(,)? ) => {
        unsafe { $crate::event::event_subscribe!($event => $ty, $callback) }
    };
    ( $event:expr , $ty:ty , $callback:expr $(,)? ) => {
        $crate::event::event_subscribe!($event => $ty, $callback)
    };
    ( $event:expr => $ty:ty , $callback:expr $(,)? ) => {{
        const __CALLBACK: fn(::std::option::Option<&$ty>) = $callback;

        extern "C-unwind" fn __event_callback_wrapper(data: *const $ty) {
            let data = unsafe { data.as_ref() };
            __CALLBACK(data)
        }

        $crate::event::event_subscribe_typed($event, __event_callback_wrapper)
    }};
}

pub use event_subscribe;
