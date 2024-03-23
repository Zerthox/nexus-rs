use crate::{addon_api, AddonApi};
use std::ffi::{c_char, c_void, CString};

pub type RawEventConsume = extern "C-unwind" fn(event_args: *const c_void);

pub type RawEventRaise =
    unsafe extern "C-unwind" fn(identifier: *const c_char, event_data: *const c_void);

pub type RawEventRaiseNotification = unsafe extern "C-unwind" fn(identifier: *const c_char);

pub type RawEventSubscribe =
    unsafe extern "C-unwind" fn(identifier: *const c_char, consume_callback: RawEventConsume);

/// Mumble identity update event identifier.
pub const MUMBLE_IDENTITY_UPDATED: &str = "EV_MUMBLE_IDENTITY_UPDATED";

/// Window resized event identifier.
pub const WINDOW_RESIZED: &str = "EV_WINDOW_RESIZED";

/// Subscribes to an event with a raw callback.
///
/// Returns a callable that reverts the subscribe.
pub fn event_subscribe_raw(
    identifier: impl AsRef<str>,
    callback: RawEventConsume,
) -> impl Fn() + Send + Sync + Clone + 'static {
    let identifier = CString::new(identifier.as_ref()).expect("failed to convert event identifier");
    let AddonApi {
        event_subscribe,
        event_unsubscribe,
        ..
    } = addon_api();
    unsafe { event_subscribe(identifier.as_ptr(), callback) };
    move || unsafe { event_unsubscribe(identifier.as_ptr(), callback) }
}

/// Unsubscribes a previously registered raw event callback.
pub fn event_unsubscribe_raw(identifier: impl AsRef<str>, callback: RawEventConsume) {
    let identifier = CString::new(identifier.as_ref()).expect("failed to convert event identifier");
    let AddonApi {
        event_unsubscribe, ..
    } = addon_api();
    unsafe { event_unsubscribe(identifier.as_ptr(), callback) }
}

/// Macro to subscribe to an event with a wrapped callback.
///
/// # Usage
/// ```no_run
/// use nexus::event;
///
/// event::subscribe!("MY_EVENT_A" => i32, |args| println!("Received {args:?}"));
///
/// const EVENT_NAME: &str = "MY_EVENT_B";
/// fn event_callback(event_args: Option<&i32>) {
///     println!("Received {event_args:?}");
/// }
/// let revert = event::subscribe!(EVENT_NAME => i32, event_callback);
/// revert();
/// ```
#[macro_export]
macro_rules! event_subscribe {
    ( $event:expr , $ty:ty , $callback:expr $(,)? ) => {
        $crate::event::subscribe!($event => $ty, $callback)
    };
    ( $event:expr => $ty:ty , $callback:expr $(,)? ) => {{
        extern "C-unwind" fn event_callback_wrapper(data: *const ::std::ffi::c_void) {
            let callback = ( $callback );
            let data = data as *const $ty;
            callback(unsafe { data.as_ref() })
        }

        $crate::event::subscribe_raw($event, event_callback_wrapper)
    }};
}

pub use event_subscribe;
