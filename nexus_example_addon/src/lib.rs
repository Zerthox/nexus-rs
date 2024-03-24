use std::ffi::{c_char, CStr};

use nexus::{
    event::event_subscribe,
    gui::{register_render, RenderType},
    imgui::Window,
    keybind::register_keybind_with_string_raw,
    paths::get_addon_dir,
    quick_access::add_shortcut,
    texture::{load_texture_from_file_raw, Texture},
    AddonFlags, UpdateProvider,
};

nexus::export! {
    name: "Example Addon",
    signature: -0x12345678, // raidcore addon id or NEGATIVE random unique signature
    load,
    unload,
    flags: AddonFlags::None,
    provider: UpdateProvider::GitHub,
    update_link: "https://github.com/zerthox/nexus-rs",
}

fn load() {
    log::info!("Loading addon");

    let addon_dir = get_addon_dir("example").expect("invalid addon dir");

    let mut show = false;
    register_render(RenderType::Render, move |ui| {
        Window::new("Test window").build(ui, || {
            if show {
                show = !ui.button("hide");
                ui.text("Hello world");
            } else {
                show = ui.button("show");
            }
        });
    });

    add_shortcut(
        "MY_SHORTCUT",
        "MY_ICON",
        "MY_ICON_HOVER",
        "MY_KEYBIND",
        "This is my tooltip text",
    )
    .revert_on_unload();

    load_texture_from_file_raw("MY_ICON", addon_dir.join("icon.png"), Some(receive_texture));
    load_texture_from_file_raw(
        "MY_ICON_HOVER",
        addon_dir.join("icon_hover.png"),
        Some(receive_texture),
    );

    register_keybind_with_string_raw("MY_KEYBIND", keybind_handler, "").revert_on_unload();

    event_subscribe!("MY_EVENT" => i32, |data| println!("received event {data:?}"))
        .revert_on_unload();
}

// TODO: callback wrapping
extern "C-unwind" fn receive_texture(identifier: *const c_char, _texture: *const Texture) {
    let identifier = unsafe { CStr::from_ptr(identifier) }.to_string_lossy();
    log::info!("texture {identifier} loaded");
}

extern "C-unwind" fn keybind_handler(identifier: *const c_char) {
    let identifier = unsafe { CStr::from_ptr(identifier) }.to_string_lossy();
    log::info!("keybind {identifier} pressed");
}

fn unload() {
    // render callbacks are unregistered automatically
    // all actions passed to on_load() or revert_on_unload() are performed
}
