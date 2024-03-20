use nexus::{
    gui::{register_render, RenderType},
    imgui::Window,
    AddonFlags, UpdateProvider,
};

nexus::export! {
    signature: -0x12345678, // raidcore addon id or NEGATIVE random unique signature
    load,
    unload: || {},
    flags: AddonFlags::None,
    provider: UpdateProvider::GitHub,
    update_link: "https://github.com/zerthox/nexus-rs",
}

fn load() {
    let mut show = false;
    log::info!("Loading addon");
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
}
