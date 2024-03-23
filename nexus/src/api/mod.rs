pub mod data_link;
pub mod event;
pub mod gui;
pub mod hook;
pub mod keybind;
pub mod localization;
pub mod log;
pub mod paths;
pub mod quick_access;
pub mod texture;
pub mod wnd_proc;

use self::{
    data_link::{RawDataGetResource, RawDataShareResource},
    event::{RawEventRaise, RawEventRaiseNotification, RawEventSubscribe},
    gui::{ImguiFree, ImguiMalloc, RawGuiAddRender, RawGuiRemRender},
    hook::{RawHookCreate, RawHookDisable, RawHookEnable, RawHookRemove},
    keybind::{RawKeybindDeregister, RawKeybindRegisterWithString, RawKeybindRegisterWithStruct},
    localization::{RawLocalizationTranslate, RawLocalizationTranslateTo},
    log::RawLog,
    paths::{RawGetAddonDir, RawGetCommonDir, RawGetGameDir},
    quick_access::{RawQuickAccessAddShortcut, RawQuickAccessAddSimple, RawQuickAccessGeneric},
    texture::{
        RawTextureGet, RawTextureGetOrCreateFromFile, RawTextureGetOrCreateFromMemory,
        RawTextureGetOrCreateFromResource, RawTextureGetOrCreateFromUrl, RawTextureLoadFromFile,
        RawTextureLoadFromMemory, RawTextureLoadFromResource, RawTextureLoadFromUrl,
    },
    wnd_proc::{RawWndProcAddRem, RawWndProcSendToGame},
};
use windows::Win32::Graphics::{Direct3D11::ID3D11Device, Dxgi::IDXGISwapChain};

/// Supported Nexus API version.
pub const API_VERSION: i32 = 2;

/// Nexus addon API.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct AddonApi {
    /// DirectX swap chain.
    pub swap_chain: *const IDXGISwapChain,

    /// ImGui context.
    pub imgui_context: *mut imgui::sys::ImGuiContext,

    /// ImGui malloc function.
    pub imgui_malloc: Option<ImguiMalloc>,

    /// ImGui free function.
    pub imgui_free: Option<ImguiFree>,

    /// Registers a new render callback.
    pub register_render: RawGuiAddRender,

    /// Removes a registered render callback.
    pub deregister_render: RawGuiRemRender,

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
    /// Synonymous to `get_addon_dir("common")`.
    pub get_common_dir: RawGetCommonDir,

    /// MinHook create.
    pub hook_create: RawHookCreate,

    /// MinHook remove.
    pub hook_remove: RawHookRemove,

    /// MinHook enable.
    pub hook_enable: RawHookEnable,

    /// MinHook disable.
    pub hook_disable: RawHookDisable,

    /// Logs a message to the log window and log file.
    ///
    /// Supports custom coloring for addon window messages, for example `<c=#FF0000>this text is red</c>`.
    pub log: RawLog,

    /// Raises an event to all subscribing addons.
    pub event_raise: RawEventRaise,

    /// Raises an event without a payload.
    ///
    /// Synonymous to `event_raise("EV_FOO", null)`.
    pub event_raise_notification: RawEventRaiseNotification,

    /// Registers a new event callback.
    pub event_subscribe: RawEventSubscribe,

    /// Removes a registered event callback.
    pub event_unsubscribe: RawEventSubscribe,

    /// Registers a new [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc) callback.
    pub register_wnd_proc: RawWndProcAddRem,

    /// Removes a registered [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc) callback.
    pub deregister_wnd_proc: RawWndProcAddRem,

    /// Sends a [WNDPROC](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc) directly to the game, bypassing other hooks.
    pub send_wnd_proc_to_game_only: RawWndProcSendToGame,

    /// Registers a new keybind handler for a given named keybind.
    ///
    /// Keybind is a string like `"ALT+SHIFT+T`.
    pub keybind_register_with_string: RawKeybindRegisterWithString,

    /// Registers a new keybind handler for a given named keybind.
    ///
    /// Keybind is a [`Keybind`](crate::keybind::Keybind) struct.
    pub keybind_register_with_struct: RawKeybindRegisterWithStruct,

    /// Removes a registered keybind.
    pub keybind_deregister: RawKeybindDeregister,

    /// Returns a pointer to the requested resource of `null` if it does not exist.
    pub get_resource: RawDataGetResource,

    /// Allocates a shared resource of the given size and returns a pointer to it for writing.
    pub share_resource: RawDataShareResource,

    /// Returns a pointer to the [`Texture`](crate::texture::Texture) or `null` if it does not exist.
    pub get_texture: RawTextureGet,

    /// Returns a pointer to the [`Texture`](crate::texture::Texture) or creates it from the file if it does not exist.
    pub get_texture_or_create_from_file: RawTextureGetOrCreateFromFile,

    /// Returns a pointer to the [`Texture`](crate::texture::Texture) or creates it from the resource if it does not exist.
    pub get_texture_or_create_from_resource: RawTextureGetOrCreateFromResource,

    /// Returns a pointer to the [`Texture`](crate::texture::Texture) or creates it from the URL if it does not exist.
    pub get_texture_or_create_from_url: RawTextureGetOrCreateFromUrl,

    /// Returns a pointer to the [`Texture`](crate::texture::Texture) or creates it from the memory if it does not exist.
    pub get_texture_or_create_from_memory: RawTextureGetOrCreateFromMemory,

    /// Creates a texture from the file and passes it to the callback when finished.
    pub load_texture_from_file: RawTextureLoadFromFile,

    /// Creates a texture from the resource and passes it to the callback when finished.
    pub load_texture_from_resource: RawTextureLoadFromResource,

    /// Creates a texture from the URL and passes it to the callback when finished.
    pub load_texture_from_url: RawTextureLoadFromUrl,

    /// Creates a texture from the memory and passes it to the callback when finished.
    pub load_texture_from_memory: RawTextureLoadFromMemory,

    /// Adds a new shortcut icon to the quick access with the given texture identifiers.
    /// When clicked the given keybind identifier will be called.
    pub add_shortcut: RawQuickAccessAddShortcut,

    /// Removes a shortcut with the given identifier from quick access.
    pub remove_shortcut: RawQuickAccessGeneric,

    /// Sends a notification icon to the given shortcut.
    pub notify_shortcut: RawQuickAccessGeneric,

    /// Adds a new ImGui callback fired when right-clicking the Nexus icon.
    pub add_simple_shortcut: RawQuickAccessAddSimple,

    /// Removes a registered simple shortcut callback.
    pub remove_simple_shortcut: RawQuickAccessGeneric,

    /// Translates the identifier into the current active language.
    /// Returns the same identifier if unavailable.
    pub translate: RawLocalizationTranslate,

    /// Translates the identifier into the given language.
    /// Returns the same identifier if unavailable.
    pub translate_to: RawLocalizationTranslateTo,
}

unsafe impl Sync for AddonApi {}

impl AddonApi {
    /// Returns the DirectX swap chain.
    #[inline]
    pub fn get_swap_chain(&self) -> Option<&IDXGISwapChain> {
        unsafe { self.swap_chain.as_ref() }
    }

    /// Retrieves the DirectX 11 device associated with the swap chain.
    #[inline]
    pub fn get_d3d11_device(&self) -> Option<ID3D11Device> {
        self.get_swap_chain()
            .and_then(|swap_chain| unsafe { swap_chain.GetDevice() }.ok())
    }
}
