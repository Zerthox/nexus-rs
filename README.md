# Raidcore Nexus bindings
Rust bindings for [Raidcore Nexus](https://raidcore.gg/Nexus) addons.

## Features
- Rust abstractions for Nexus Addon API
- Wrapping custom callbacks via macros 
- ImGui interfacing via [imgui-rs](https://github.com/imgui-rs/imgui-rs)
- Optional logging via [log](https://github.com/rust-lang/log)
- Optional [serde](https://serde.rs) and [strum](https://github.com/Peternator7/strum) integration
- Optional bindings for GW2 Mumble API

## Usage
```toml
nexus = { git = "https://github.com/zerthox/nexus-rs" }
```

```rs
use nexus::gui::{register_render, render, RenderType};

nexus::export! {
    name: "My Addon",
    signature: -0x12345678,
    load: || {
        register_render(RenderType::Render, render!(|ui| {
            Window::new("My Window").build(ui, || {
                ui.text("Hello World");
            });
        }));
    },
}
```
 