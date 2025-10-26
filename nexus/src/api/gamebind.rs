//! Game keybinds.

use crate::{AddonApi, GameBindApi};

/// Game keybinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIter,
        strum::IntoStaticStr,
        strum::VariantArray,
        strum::VariantNames
    )
)]
#[repr(C)]
pub enum GameBind {
    // Movement
    MoveForward = 0,
    MoveBackward = 1,
    MoveLeft = 2,
    MoveRight = 3,
    MoveTurnLeft = 4,
    MoveTurnRight = 5,
    MoveDodge = 6,
    MoveAutoRun = 7,
    MoveWalk = 8,
    MoveJump = 9,
    MoveSwimUp = 10,
    MoveSwimDown = 11,
    MoveAboutFace = 12,

    // Skills
    SkillWeaponSwap = 17,
    SkillWeapon1 = 18,
    SkillWeapon2 = 19,
    SkillWeapon3 = 20,
    SkillWeapon4 = 21,
    SkillWeapon5 = 22,
    SkillHeal = 23,
    SkillUtility1 = 24,
    SkillUtility2 = 25,
    SkillUtility3 = 26,
    SkillElite = 27,
    SkillProfession1 = 28,
    SkillProfession2 = 29,
    SkillProfession3 = 30,
    SkillProfession4 = 31,
    SkillProfession5 = 79,
    SkillProfession6 = 201,
    SkillProfession7 = 202,
    SkillSpecialAction = 82,

    // Targeting
	TargetAlert = 131,
	TargetCall = 32,
	TargetTake = 33,
	TargetCallLocal = 199,
	TargetTakeLocal = 200,
	TargetEnemyNearest = 34,
	TargetEnemyNext = 35,
	TargetEnemyPrev = 36,
	TargetAllyNearest = 37,
	TargetAllyNext = 38,
	TargetAllyPrev = 39,
	TargetLock = 40,
	TargetSnapGroundTarget = 80,
	TargetSnapGroundTargetToggle = 115,
	TargetAutoTargetingDisable = 116,
	TargetAutoTargetingToggle = 117,
	TargetAllyTargetingMode = 197,
	TargetAllyTargetingModeToggle = 198,

    // UI Binds
    UiTradingPost = 41,
    UiContacts = 42,
    UiGuild = 43,
    UiHero = 44,
    UiInventory = 45,
    UiPets = 46,
    UiLogout = 47,
    UiMail = 71,
    UiOptions = 48,
    UiParty = 49,
    UiPvp = 73,
    UiPvpBuild = 75,
    UiScoreboard = 50,
    UiWizardsVault = 209,
    UiInformation = 51,
    UiChatToggle = 70,
    UiChatCommand = 52,
    UiChatFocus = 53,
    UiChatReply = 54,
    UiToggle = 55,
    UiSquadBroadcastChatToggle = 85,
    UiSquadBroadcastChatCommand = 83,
    UiSquadBroadcastChatFocus = 84,

    // Camera
	CameraFree = 13,
	CameraZoomIn = 14,
	CameraZoomOut = 15,
	CameraReverse = 16,
	CameraActionMode = 78,
	CameraActionModeDisable = 114,

    // Screenshots
    ScreenshotNormal = 56,
    ScreenshotStereoscopic = 57,

    // Map
	MapToggle = 59,
	MapFocusPlayer = 60,
	MapFloorDown = 61,
	MapFloorUp = 62,
	MapZoomIn = 63,
	MapZoomOut = 64,

    // Mounts
    MountToggle = 152,
    MountMovement = 130,
    MountSecondaryMovement = 153,
    MountRaptor = 155,
    MountSpringer = 156,
    MountSkimmer = 157,
    MountJackal = 158,
    MountGriffon = 159,
    MountRollerBeetle = 161,
    MountWarclaw = 169,
    MountSkyscale = 170,
    MountSiegeTurtle = 203,

    // Spectator Binds
    SpectatorNearestFixed = 102,
    SpectatorNearestPlayer = 103,
    SpectatorPlayerRed1 = 104,
    SpectatorPlayerRed2 = 105,
    SpectatorPlayerRed3 = 106,
    SpectatorPlayerRed4 = 107,
    SpectatorPlayerRed5 = 108,
    SpectatorPlayerBlue1 = 109,
    SpectatorPlayerBlue2 = 110,
    SpectatorPlayerBlue3 = 111,
    SpectatorPlayerBlue4 = 112,
    SpectatorPlayerBlue5 = 113,
    SpectatorFreeCamera = 120,
    SpectatorFreeCameraMode = 127,
    SpectatorFreeMoveForward = 121,
    SpectatorFreeMoveBackward = 122,
    SpectatorFreeMoveLeft = 123,
    SpectatorFreeMoveRight = 124,
    SpectatorFreeMoveUp = 125,
    SpectatorFreeMoveDown = 126,

    // Squad Markers
    SquadMarkerPlaceWorldArrow = 86,
    SquadMarkerPlaceWorldCircle = 87,
    SquadMarkerPlaceWorldHeart = 88,
    SquadMarkerPlaceWorldSquare = 89,
    SquadMarkerPlaceWorldStar = 90,
    SquadMarkerPlaceWorldSwirl = 91,
    SquadMarkerPlaceWorldTriangle = 92,
    SquadMarkerPlaceWorldCross = 93,
    SquadMarkerClearAllWorld = 119,
    SquadMarkerSetAgentArrow = 94,
    SquadMarkerSetAgentCircle = 95,
    SquadMarkerSetAgentHeart = 96,
    SquadMarkerSetAgentSquare = 97,
    SquadMarkerSetAgentStar = 98,
    SquadMarkerSetAgentSwirl = 99,
    SquadMarkerSetAgentTriangle = 100,
    SquadMarkerSetAgentCross = 101,
    SquadMarkerClearAllAgent = 118,

    // Mastery Skills
    MasteryAccess = 196,
    MasteryAccessFishing = 204,
    MasteryAccessSkiff = 205,
    MasteryAccessJadeBotWaypoint = 206,
    MasteryAccessRiftScan = 207,
    MasteryAccessSkyscale = 208,
    MasteryAccessHomesteadDoorway = 211,

    // Miscellaneous Binds
    MiscAoELoot = 74,
    MiscInteract = 65,
    MiscShowEnemies = 66,
    MiscShowAllies = 67,
    MiscStowDrawWeapon = 68,
    MiscToggleLanguage = 69,
    MiscTogglePetCombat = 76,
    MiscToggleFullScreen = 160,
    MiscToggleDecorationMode = 210,

    // Toys/Novelties
    ToyUseDefault = 162,
    ToyUseSlotChair = 163,
    ToyUseSlotInstrument = 164,
    ToyUseSlotHeltItem = 165,
    ToyUseSlotToy = 166,
    ToyUseSlotTonic = 167,

    // Build Templates
    Loadout1 = 171,
    Loadout2 = 172,
    Loadout3 = 173,
    Loadout4 = 174,
    Loadout5 = 175,
    Loadout6 = 176,
    Loadout7 = 177,
    Loadout8 = 178,
    Loadout9 = 179,

    // Equipment Templates
    GearLoadout1 = 182,
    GearLoadout2 = 183,
    GearLoadout3 = 184,
    GearLoadout4 = 185,
    GearLoadout5 = 186,
    GearLoadout6 = 187,
    GearLoadout7 = 188,
    GearLoadout8 = 189,
    GearLoadout9 = 190,
}

pub type RawGamebindPressAsync = unsafe extern "C-unwind" fn(game_bind: GameBind);

pub type RawGamebindReleaseAsync = unsafe extern "C-unwind" fn(game_bind: GameBind);

pub type RawGamebindInvokeAsync = unsafe extern "C-unwind" fn(game_bind: GameBind, duration: i32);

pub type RawGamebindPress = unsafe extern "C-unwind" fn(game_bind: GameBind);

pub type RawGamebindRelease = unsafe extern "C-unwind" fn(game_bind: GameBind);

pub type RawGamebindIsBound = unsafe extern "C-unwind" fn(game_bind: GameBind) -> bool;

/// Presses the given [`GameBind`] asynchronously.
#[inline]
pub fn press_gamebind_async(bind: GameBind) {
    let GameBindApi { press_async, .. } = AddonApi::get().game_bind;
    unsafe { press_async(bind) }
}

/// Releases the given [`GameBind`] asynchronously.
#[inline]
pub fn release_gamebind_async(bind: GameBind) {
    let GameBindApi { release_async, .. } = AddonApi::get().game_bind;
    unsafe { release_async(bind) }
}

/// Presses the given [`GameBind`] asynchronously and release after the given duration.
#[inline]
pub fn invoke_gamebind_async(bind: GameBind, duration: i32) {
    let GameBindApi { invoke_async, .. } = AddonApi::get().game_bind;
    unsafe { invoke_async(bind, duration) }
}

/// Presses the given [`GameBind`].
#[inline]
pub fn press_gamebind(bind: GameBind) {
    let GameBindApi { press, .. } = AddonApi::get().game_bind;
    unsafe { press(bind) }
}

/// Releases the given [`GameBind`].
#[inline]
pub fn release_gamebind(bind: GameBind) {
    let GameBindApi { release, .. } = AddonApi::get().game_bind;
    unsafe { release(bind) }
}

/// Checks if the given [`GameBind`] is bound.
#[inline]
pub fn is_gamebind_bound(bind: GameBind) -> bool {
    let GameBindApi { is_bound, .. } = AddonApi::get().game_bind;
    unsafe { is_bound(bind) }
}
