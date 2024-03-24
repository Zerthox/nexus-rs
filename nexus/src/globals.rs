use crate::{
    api::AddonApi,
    gui,
    log::{log, LogLevel},
};
use std::{
    fmt, mem, panic, ptr,
    sync::{Mutex, OnceLock},
};

#[cfg(feature = "log")]
use crate::logger::NexusLogger;

static ADDON_API: OnceLock<&'static AddonApi> = OnceLock::new();

static IMGUI_CTX: OnceLock<ContextWrapper> = OnceLock::new();

static IMGUI_UI: OnceLock<UiWrapper> = OnceLock::new();

/// Initializes globals.
///
/// Any calls after the initial one will result in a panic.
/// A call to this is inserted automatically by the [`export`](crate::export) macro.
///
/// # Safety
/// The passed pointer must be a valid [`AddonApi`] with `'static` lifetime.
pub unsafe fn init(api: *const AddonApi, addon_name: &'static str) {
    let api = api.as_ref().expect("no addon api supplied");
    ADDON_API
        .set(api)
        .expect("addon api initialized multiple times");

    // panic hook
    panic::set_hook(Box::new(move |info| {
        log(LogLevel::Critical, addon_name, info.to_string())
    }));

    // init logger
    #[cfg(feature = "log")]
    NexusLogger::set_logger(addon_name);

    // setup imgui
    imgui::sys::igSetCurrentContext(api.imgui_context);
    imgui::sys::igSetAllocatorFunctions(api.imgui_malloc, api.imgui_free, ptr::null_mut());
    IMGUI_CTX
        .set(imgui::Context::current().into())
        .expect("imgui context initialized multiple times");
    let ctx = &IMGUI_CTX.get().unwrap_unchecked().0;
    IMGUI_UI
        .set(imgui::Ui::from_ctx(ctx).into())
        .expect("imgui ui initialized multiple times");
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
///  A call to this is inserted automatically by the [`export`](crate::export) macro.
///
/// # Safety
/// This may perform not thread-safe operations and leave globals in an invalid state.
pub unsafe fn deinit() {
    // remove gui callbacks
    gui::unregister_all();

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

/// Returns an [`imgui::Ui`] for rendering a frame.
///
/// # Safety
/// It is not safe to share [`imgui::Ui`] between threads.
#[inline]
pub unsafe fn ui() -> &'static imgui::Ui<'static> {
    &IMGUI_UI.get().expect("imgui not initialized").0
}

/// Helper to store [`imgui::Ui`] as a global
#[repr(transparent)]
struct UiWrapper(pub imgui::Ui<'static>);

impl From<imgui::Ui<'static>> for UiWrapper {
    #[inline]
    fn from(ui: imgui::Ui<'static>) -> Self {
        Self(ui)
    }
}

impl fmt::Debug for UiWrapper {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

unsafe impl Send for UiWrapper {}

unsafe impl Sync for UiWrapper {}

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
