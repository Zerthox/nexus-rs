use std::ffi::{c_char, c_void};

pub type RawEventConsume = unsafe extern "C-unwind" fn(event_args: *const c_void);

pub type RawEventRaise =
    unsafe extern "C-unwind" fn(identifier: *const c_char, event_data: *const c_void);

pub type RawEventRaiseNotification = unsafe extern "C-unwind" fn(identifier: *const c_char);

pub type RawEventSubscribe =
    unsafe extern "C-unwind" fn(identifier: *const c_char, consume_callback: RawEventConsume);
