use crate::{
    api::AddonApi,
    log::{log, LogLevel},
};
use once_cell::sync::OnceCell;
use std::{fmt, panic, ptr, sync::OnceLock};

#[cfg(feature = "log")]
use crate::logger::NexusLogger;

static ADDON_API: OnceLock<&'static AddonApi> = OnceLock::new();

static IMGUI_CTX: OnceCell<ContextWrapper> = OnceCell::new();

/// Initializes globals.
///
/// Any calls after the initial one will result in a panic.
/// A call to this is inserted automatically by the [`export`](crate::export) macro.
///
/// # Safety
/// The passed pointer must be a valid [`AddonApi`] with `'static` lifetime.
pub unsafe fn init(api: *const AddonApi) {
    let api = api.as_ref().expect("no addon api supplied");
    ADDON_API
        .set(api)
        .expect("addon api initialized multiple times");

    // panic hook
    panic::set_hook(Box::new(move |info| {
        log(LogLevel::Critical, "file", format!("error: {info}"))
    }));

    // init logger
    #[cfg(feature = "log")]
    NexusLogger::set_logger();

    // setup imgui
    imgui::sys::igSetCurrentContext(api.imgui_context);
    imgui::sys::igSetAllocatorFunctions(api.imgui_malloc, api.imgui_free, ptr::null_mut());
    IMGUI_CTX
        .set(imgui::Context::current().into())
        .expect("imgui context initialized multiple times");
}

/// Returns the Nexus [`AddonApi`] instance.
///
/// Panics if called before initialization.
#[inline]
pub fn addon_api() -> &'static AddonApi {
    ADDON_API.get().expect("addon api not initialized")
}

/// Creates an [`imgui::Ui`] for rendering a frame.
///
/// # Safety
/// The caller must ensure this is only called after globals have been initialized
/// and ensure an appropriate lifetime for the returned [`imgui::Ui`].
#[inline]
pub unsafe fn ui<'a>() -> imgui::Ui<'a> {
    imgui::Ui::from_ctx(&IMGUI_CTX.get().unwrap_unchecked().0)
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
