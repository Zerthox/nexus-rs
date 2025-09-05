# Raidcore Nexus bindings
Rust bindings for [Raidcore Nexus](https://github.com/RaidcoreGG/Nexus) addons ([website](https://raidcore.gg/Nexus)).
Documentation available at [zerthox.github.io/nexus-rs/nexus](https://zerthox.github.io/nexus-rs/nexus).

- Rust abstractions for the [Nexus Addon API](https://github.com/RaidcoreGG/RCGG-lib-nexus-api)
- Wrapping custom callbacks via macros 
- [ImGui](https://github.com/ocornut/imgui) interfacing via [imgui-rs](https://github.com/imgui-rs/imgui-rs)
- Optional logging via [log](https://github.com/rust-lang/log)
- Optional [serde](https://serde.rs) and [strum](https://github.com/Peternator7/strum) integration
- Optional bindings for the GW2 Mumble API
- Optional bindings for events forwarded from [ArcDPS](https://deltaconnected.com/arcdps/) & [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases).
- Optional bindings for [Realtime API](https://github.com/RaidcoreGG/GW2-RealTime-API-Releases)
- Optional [MinHook](https://github.com/TsudaKageyu/minhook) bindings with interfaces from [retour-rs](https://github.com/Hpmason/retour-rs)

## Usage
```toml
nexus = { git = "https://github.com/zerthox/nexus-rs" }
```

```rs
use nexus::{
    gui::{register_render, render, RenderType},
    imgui::Window,
};

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

## Features
| Feature | Description |
| --- | --- |
| arc | Enable [ArcDPS](https://deltaconnected.com/arcdps/) support *(alias: arcdps, evtc)* |
| extras | Enable [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) support |
| hook | Enable [MinHook](https://github.com/TsudaKageyu/minhook) bindings |
| log | Enable [log](https://github.com/rust-lang/log) support |
| log_filter | Enable log filter (large binary size!) |
| mumble | Enable Mumble link support |
| mumble_json | Enable Mumble identity JSON parsing |
| panic | Enable panic hook to log panics to arcdps.log *(enabled by default)* |
| panic_trace | Enable capturing backtrace in panic hook *(enabled by default)* |
| panic_msgbox| Enable showing message box in panic hook *(enabled by default)* |
| rtapi | Enable [RealTime API](https://github.com/RaidcoreGG/GW2-RealTime-API-Releases) support |
| serde | Enable [serde](https://serde.rs) support |
| strum | Enable [strum](https://github.com/Peternator7/strum) support |
