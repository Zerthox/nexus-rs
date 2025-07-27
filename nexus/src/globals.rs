use crate::{api::AddonApi, imgui};
use std::{
    fmt, mem, ptr,
    sync::{Mutex, OnceLock},
};

#[cfg(feature = "panic")]
use crate::panic::init_panic_hook;

#[cfg(feature = "log")]
use crate::logger::NexusLogger;

static ADDON_API: OnceLock<&'static AddonApi> = OnceLock::new();

static IMGUI_CTX: OnceLock<ContextWrapper> = OnceLock::new();

thread_local! { static IMGUI_UI: imgui::Ui<'static> = imgui::Ui::from_ctx(&IMGUI_CTX.get().expect("imgui context not initialized").0); }

/// Initializes globals.
///
/// Any calls after the initial one will result in a panic.
/// A call to this is inserted automatically by the [`export`](crate::export) macro.
///
/// # Safety
/// The passed pointer must be a valid [`AddonApi`] with `'static` lifetime.
pub unsafe fn init(
    api: *const AddonApi,
    addon_name: &'static str,
    _log_filter: Option<&'static str>,
) {
    let api = api.as_ref().expect("no addon api supplied");
    ADDON_API
        .set(api)
        .expect("addon api initialized multiple times");

    // panic hook
    #[cfg(feature = "panic")]
    init_panic_hook(addon_name);

    // init logger
    #[cfg(feature = "log")]
    NexusLogger::set_logger(addon_name, _log_filter);

    // setup imgui
    imgui::sys::igSetCurrentContext(api.imgui_context);
    imgui::sys::igSetAllocatorFunctions(api.imgui_malloc, api.imgui_free, ptr::null_mut());
    IMGUI_CTX
        .set(imgui::Context::current().into())
        .expect("imgui context initialized multiple times");
}

/// Actions to be performed on addon unload.
static UNLOAD_ACTIONS: Mutex<Vec<Box<dyn FnOnce() + Send>>> = Mutex::new(Vec::new());

/// Adds a new action to be perform on unload.
#[inline]
pub fn on_unload(action: impl FnOnce() + Send + 'static) {
    UNLOAD_ACTIONS.lock().unwrap().push(Box::new(action));
}

/// Cleans up during addon unload.
///
/// A call to this is inserted automatically by the [`export`](crate::export) macro.
///
/// # Safety
/// This may perform not thread-safe operations and leave globals in an invalid state.
pub unsafe fn deinit() {
    // perform stored unload actions
    let mut guard = UNLOAD_ACTIONS.lock().unwrap();
    let vec: Vec<_> = mem::take(&mut guard);
    for action in vec {
        action();
    }
}

/// Returns the Nexus [`AddonApi`] instance.
///
/// Panics if called before initialization.
#[inline]
pub fn addon_api() -> &'static AddonApi {
    ADDON_API.get().expect("addon api not initialized")
}

/// Retrieves the [`imgui::Ui`] for rendering a frame.
///
/// # Safety
/// The [`imgui::Ui`] should only be accessed in render thread.
#[inline]
pub unsafe fn with_ui<R>(body: impl FnOnce(&imgui::Ui<'static>) -> R) -> R {
    IMGUI_UI.with(body)
}

/// Helper to store [`imgui::Context`] as a global.
#[repr(transparent)]
struct ContextWrapper(pub imgui::Context);

impl From<imgui::Context> for ContextWrapper {
    #[inline]
    fn from(ctx: imgui::Context) -> Self {
        Self(ctx)
    }
}

impl fmt::Debug for ContextWrapper {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

unsafe impl Send for ContextWrapper {}

unsafe impl Sync for ContextWrapper {}
