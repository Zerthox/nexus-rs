use nexus::{AddonFlags, UpdateProvider};

nexus::export! {
    signature: -0x12345678, // raidcore addon id or NEGATIVE random unique signature
    load: || {},
    unload: || {},
    flags: AddonFlags::None,
    provider: UpdateProvider::GitHub,
    update_link: "https://github.com/zerthox/nexus-rs",
}
