pub mod data;
pub mod event;
pub mod gui;
pub mod hook;
pub mod keybind;
pub mod link;
pub mod localization;
pub mod log;
pub mod paths;
pub mod quick_access;
pub mod texture;

use self::{
    data::{RawDataGetResource, RawDataShareResource},
    event::{RawEventRaise, RawEventRaiseNotification, RawEventSubscribe},
    gui::{ImguiFree, ImguiMalloc, RawGuiAddRender, RawGuiRemRender},
    hook::{RawHookCreate, RawHookDisable, RawHookEnable, RawHookRemove},
    keybind::{
        RawKeybindDeregister, RawKeybindRegisterWithString, RawKeybindRegisterWithStruct,
        RawWndProcAddRem, RawWndProcSendToGame,
    },
    localization::{LocalizationTranslate, LocalizationTranslateTo},
    log::RawLog,
    paths::{RawGetAddonDir, RawGetCommonDir, RawGetGameDir},
    quick_access::{RawQuickAccessAddShortcut, RawQuickAccessAddSimple, RawQuickAccessGeneric},
    texture::{
        RawTextureGet, RawTextureGetOrCreateFromFile, RawTextureGetOrCreateFromMemory,
        RawTextureGetOrCreateFromResource, RawTextureGetOrCreateFromUrl, RawTextureLoadFromFile,
        RawTextureLoadFromMemory, RawTextureLoadFromResource, RawTextureLoadFromUrl,
    },
};
use windows::Win32::Graphics::{Direct3D11::ID3D11Device, Dxgi::IDXGISwapChain};

pub const API_VERSION: i32 = 2;

// TODO: provide access api functions

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AddonApi {
    pub swap_chain: *const IDXGISwapChain,
    pub imgui_context: *mut imgui::sys::ImGuiContext,
    pub imgui_malloc: Option<ImguiMalloc>,
    pub imgui_free: Option<ImguiFree>,

    pub register_render: RawGuiAddRender,
    pub deregister_render: RawGuiRemRender,

    pub get_game_dir: RawGetGameDir,
    pub get_addon_dir: RawGetAddonDir,
    pub get_common_dir: RawGetCommonDir,

    pub hook_create: RawHookCreate,
    pub hook_remove: RawHookRemove,
    pub hook_enable: RawHookEnable,
    pub hook_disable: RawHookDisable,

    pub log: RawLog,

    pub event_raise: RawEventRaise,
    pub event_raise_notification: RawEventRaiseNotification,
    pub event_subscribe: RawEventSubscribe,
    pub event_unsubscribe: RawEventSubscribe,

    pub register_wnd_proc: RawWndProcAddRem,
    pub deregister_wnd_proc: RawWndProcAddRem,
    pub send_wnd_proc_to_game_only: RawWndProcSendToGame,

    pub keybind_register_with_string: RawKeybindRegisterWithString,
    pub keybind_register_with_struct: RawKeybindRegisterWithStruct,
    pub keybind_deregister: RawKeybindDeregister,

    pub get_resource: RawDataGetResource,
    pub share_resource: RawDataShareResource,

    pub get_texture: RawTextureGet,
    pub get_texture_or_create_from_file: RawTextureGetOrCreateFromFile,
    pub get_texture_or_create_from_resource: RawTextureGetOrCreateFromResource,
    pub get_texture_or_create_from_url: RawTextureGetOrCreateFromUrl,
    pub get_texture_or_create_from_memory: RawTextureGetOrCreateFromMemory,
    pub load_texture_from_file: RawTextureLoadFromFile,
    pub load_texture_from_resource: RawTextureLoadFromResource,
    pub load_texture_from_url: RawTextureLoadFromUrl,
    pub load_texture_from_memory: RawTextureLoadFromMemory,

    pub add_shortcut: RawQuickAccessAddShortcut,
    pub remove_shortcut: RawQuickAccessGeneric,
    pub notify_shortcut: RawQuickAccessGeneric,
    pub add_simple_shortcut: RawQuickAccessAddSimple,
    pub remove_simple_shortcut: RawQuickAccessGeneric,

    pub translate: LocalizationTranslate,
    pub translate_to: LocalizationTranslateTo,
}

impl AddonApi {
    pub fn get_swap_chain(&self) -> Option<&IDXGISwapChain> {
        unsafe { self.swap_chain.as_ref() }
    }

    pub fn get_d3d11_device(&self) -> Option<ID3D11Device> {
        self.get_swap_chain()
            .and_then(|swap_chain| unsafe { swap_chain.GetDevice() }.ok())
    }
}
