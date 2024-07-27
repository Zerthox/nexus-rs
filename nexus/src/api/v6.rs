//! Addon API version 4.

use super::{
    alert::RawAlertNotify,
    data_link::{RawDataGetResource, RawDataShareResource},
    event::{
        RawEventRaise, RawEventRaiseNotification, RawEventRaiseNotificationTargeted,
        RawEventRaiseTargeted, RawEventSubscribe,
    },
    font::{
        RawFontAddFromFile, RawFontAddFromMemory, RawFontAddFromResource, RawFontGet,
        RawFontRelease, RawFontResize,
    },
    gui::{ImguiFree, ImguiMalloc, RawGuiAddRender, RawGuiRegisterCloseOnEscape, RawGuiRemRender},
    hook::{RawHookCreate, RawHookDisable, RawHookEnable, RawHookRemove},
    keybind::{
        RawKeybindDeregister, RawKeybindInvoke, RawKeybindRegisterWithString,
        RawKeybindRegisterWithStruct,
    },
    localization::{RawLocalizationSet, RawLocalizationTranslate, RawLocalizationTranslateTo},
    log::RawLog,
    paths::{RawGetAddonDir, RawGetCommonDir, RawGetGameDir},
    quick_access::{
        RawQuickAccessAddContextMenu, RawQuickAccessAddShortcut, RawQuickAccessGeneric,
    },
    texture::{
        RawTextureGet, RawTextureGetOrCreateFromFile, RawTextureGetOrCreateFromMemory,
        RawTextureGetOrCreateFromResource, RawTextureGetOrCreateFromUrl, RawTextureLoadFromFile,
        RawTextureLoadFromMemory, RawTextureLoadFromResource, RawTextureLoadFromUrl,
    },
    updater::RawRequestUpdate,
    wnd_proc::{RawWndProcAddRem, RawWndProcSendToGame},
};
use windows::Win32::Graphics::{Direct3D11::ID3D11Device, Dxgi::IDXGISwapChain};

/// Nexus addon API (version 6).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct AddonApi {
    /// DirectX swap chain.
    pub swap_chain: IDXGISwapChain,

    /// ImGui context.
    pub imgui_context: *mut imgui::sys::ImGuiContext,

    /// ImGui malloc function.
    pub imgui_malloc: Option<ImguiMalloc>,

    /// ImGui free function.
    pub imgui_free: Option<ImguiFree>,

    /// Rendering API.
    pub renderer: RendererApi,

    /// Downloads the addon available at remote without checking its version.
    pub request_update: RawRequestUpdate,

    /// Logs a message to the log window and log file.
    ///
    /// Supports custom coloring for addon window messages, for example `<c=#FF0000>this text is red</c>`.
    pub log: RawLog,

    /// Ui API.
    pub ui: UiApi,

    /// Paths API.
    pub path: PathApi,

    /// MinHook API.
    pub min_hook: MinHookApi,

    /// Event API.
    pub event: EventApi,

    /// [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc) API.
    pub wnd_proc: WndProcApi,

    /// Input keybinds API.
    pub input_binds: InputBindsApi,

    /// Game keybinds API.
    pub game_binds: GameBindsApi,

    /// Data Link API.
    pub data_link: DataLinkApi,

    /// Texture Api.
    pub texture: TextureApi,

    /// Quick Access API.
    pub quick_access: QuickAccessApi,

    /// Localization API.
    pub localization: LocalizationApi,

    /// Font API.
    pub font: FontApi,
}

unsafe impl Sync for AddonApi {}

unsafe impl Send for AddonApi {}

impl AddonApi {
    /// Nexus Addon API version.
    pub const VERSION: i32 = 6;

    /// Retrieves the DirectX 11 device associated with the swap chain.
    #[inline]
    pub fn get_d3d11_device(&self) -> Option<ID3D11Device> {
        unsafe { self.swap_chain.GetDevice() }.ok()
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RendererApi {
    /// Registers a new render callback.
    pub register: RawGuiAddRender,

    /// Removes a registered render callback.
    pub deregister: RawGuiRemRender,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UiApi {
    /// Sends a text alert to the user visible for a short amount of time.
    pub send_alert: RawAlertNotify,

    /// Registers a window name to get its bool toggled when escape is pressed.
    pub register_close_on_escape: RawGuiRegisterCloseOnEscape,

    /// Deregisters a window name to listen to on escape.
    pub deregister_close_on_escape: RawGuiRegisterCloseOnEscape,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PathApi {
    /// Returns the path to the game directory.
    ///
    /// For example `"C:\Program Files\Guild Wars 2\"`.
    pub get_game_dir: RawGetGameDir,

    /// Returns a path to `"\addons\{name}"`.
    ///
    /// Passing `null` or `""` returns `"\addons"` without trailing slash.
    pub get_addon_dir: RawGetAddonDir,

    /// Returns the path to the common addon folder.
    ///
    /// Alias for `get_addon_dir("common")`.
    pub get_common_dir: RawGetCommonDir,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MinHookApi {
    /// MinHook create.
    pub create: RawHookCreate,

    /// MinHook remove.
    pub remove: RawHookRemove,

    /// MinHook enable.
    pub enable: RawHookEnable,

    /// MinHook disable.
    pub disable: RawHookDisable,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct EventApi {
    /// Raises an event to all subscribing addons.
    pub raise: RawEventRaise,

    /// Raises an event without payload to all subscribing addons.
    ///
    /// Alias for `event_raise("EV_FOO", null)`.
    pub raise_notification: RawEventRaiseNotification,

    /// Raises an event for a specific subscribing addon.
    pub raise_targeted: RawEventRaiseTargeted,

    /// Raises an event without payload for a specific subscribing addon.
    ///
    /// Alias for `event_raise_targeted("EV_FOO", null)`.
    pub raise_notification_targeted: RawEventRaiseNotificationTargeted,

    /// Registers a new event callback.
    pub subscribe: RawEventSubscribe,

    /// Removes a registered event callback.
    pub unsubscribe: RawEventSubscribe,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct WndProcApi {
    /// Registers a new [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc) callback.
    pub register: RawWndProcAddRem,

    /// Removes a registered [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc) callback.
    pub deregister: RawWndProcAddRem,

    /// Sends a [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc) directly to the game, bypassing other hooks.
    pub send_to_game_only: RawWndProcSendToGame,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct InputBindsApi {
    /// Trigger a keybind programmatically.
    pub invoke: RawKeybindInvoke,

    /// Registers a new keybind handler for a given named keybind.
    ///
    /// Keybind is a string like `"ALT+SHIFT+T`.
    pub register_with_string: RawKeybindRegisterWithString,

    /// Registers a new keybind handler for a given named keybind.
    ///
    /// Keybind is a [`Keybind`](crate::keybind::Keybind) struct.
    pub register_with_struct: RawKeybindRegisterWithStruct,

    /// Removes a registered keybind.
    pub deregister: RawKeybindDeregister,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct GameBindsApi {
    /// Presses the keys of a given bind.
    pub press_async: *mut (),

    /// Releases the keypress of a given bind.
    pub release_async: *mut (),

    /// Sends the keys of a given bind and then releases them after a given duration.
    pub invoke_async: *mut (),

    /// TPresses the keys of a given bind.
    pub press: *mut (),

    /// Releases the keypress of a given bind.
    pub release: *mut (),

    /// Returns if a given game bind is set.
    pub is_bound: *mut (),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DataLinkApi {
    /// Returns a pointer to the requested resource of `null` if it does not exist.
    pub get: RawDataGetResource,

    /// Allocates a shared resource of the given size and returns a pointer to it for writing.
    pub share: RawDataShareResource,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TextureApi {
    /// Returns a pointer to the [`Texture`](crate::texture::Texture) or `null` if it does not exist.
    pub get: RawTextureGet,

    /// Returns a pointer to the [`Texture`](crate::texture::Texture) or creates it from the file if it does not exist.
    pub get_or_create_from_file: RawTextureGetOrCreateFromFile,

    /// Returns a pointer to the [`Texture`](crate::texture::Texture) or creates it from the resource if it does not exist.
    pub get_or_create_from_resource: RawTextureGetOrCreateFromResource,

    /// Returns a pointer to the [`Texture`](crate::texture::Texture) or creates it from the URL if it does not exist.
    pub get_or_create_from_url: RawTextureGetOrCreateFromUrl,

    /// Returns a pointer to the [`Texture`](crate::texture::Texture) or creates it from the memory if it does not exist.
    pub get_or_create_from_memory: RawTextureGetOrCreateFromMemory,

    /// Creates a texture from the file and passes it to the callback when finished.
    pub load_from_file: RawTextureLoadFromFile,

    /// Creates a texture from the resource and passes it to the callback when finished.
    pub load_from_resource: RawTextureLoadFromResource,

    /// Creates a texture from the URL and passes it to the callback when finished.
    pub load_from_url: RawTextureLoadFromUrl,

    /// Creates a texture from the memory and passes it to the callback when finished.
    pub load_from_memory: RawTextureLoadFromMemory,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct QuickAccessApi {
    /// Adds a new shortcut icon to the quick access with the given texture identifiers.
    /// When clicked the given keybind identifier will be called.
    pub add: RawQuickAccessAddShortcut,

    /// Removes a shortcut with the given identifier from quick access.
    pub remove: RawQuickAccessGeneric,

    /// Sends a notification icon to the given shortcut.
    pub notify: RawQuickAccessGeneric,

    /// Adds a new ImGui callback fired when right-clicking the Nexus icon.
    pub add_context_menu: RawQuickAccessAddContextMenu,

    /// Removes a registered simple shortcut callback.
    pub remove_context_menu: RawQuickAccessGeneric,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct LocalizationApi {
    /// Translates the identifier into the current active language.
    /// Returns the same identifier if unavailable.
    pub translate: RawLocalizationTranslate,

    /// Translates the identifier into the given language.
    /// Returns the same identifier if unavailable.
    pub translate_to: RawLocalizationTranslateTo,

    /// Set a translated string at runtime.
    pub set: RawLocalizationSet,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FontApi {
    /// Requests a font to be sent to the given callback/receiver.
    pub get: RawFontGet,

    /// Releases a callback/receiver from a specific font.
    pub release: RawFontRelease,

    /// Adds a font from disk and sends updates to the callback.
    pub add_from_file: RawFontAddFromFile,

    /// Adds a font from an embedded resource and sends updates to the callback.
    pub add_from_resource: RawFontAddFromResource,

    /// Adds a font from memory and sends updates to the callback.
    pub add_from_memory: RawFontAddFromMemory,

    /// Resizes a font and sends updates to the registered callbacks.
    pub resize: RawFontResize,
}
