use super::{AgentType, Attitude, BreakbarState, RealTimeData};

/// Currently selected target.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TargetData {
    /// Agent id of the target, same as in the combat events.
    pub id: u32,

    /// Agent type of the target.
    pub agent_type: Result<AgentType, u32>,

    /// Species id for characters, gadget id for gadgets and attack targets.
    ///
    /// Same as [`CombatAgent::species_id`](super::CombatAgent::species_id).
    pub species_id: u32,

    /// Attitude of the target.
    pub attitude: Result<Attitude, u32>,

    /// Health of the target in percent, 0-100.
    ///
    /// Currently whole numbers, as the game shows them.
    pub health: f32,

    /// Barrier of the target in percent of max health, 0-100.
    ///
    /// Currently whole numbers, as the game shows them.
    pub barrier: f32,

    /// Breakbar state of the target.
    pub breakbar_state: Result<BreakbarState, u32>,

    /// Breakbar of the target in percent, 0-100.
    ///
    /// Currently whole numbers, as the game shows them.
    pub breakbar: f32,
}

impl TargetData {
    /// Reads target data from the given data pointer.
    ///
    /// Returns [`None`] if nothing is selected.
    ///
    /// # Safety
    /// The pointer must be safe to read from and RTAPI must provide the target fields.
    pub unsafe fn read(data: *const RealTimeData) -> Option<Self> {
        unsafe {
            let id = (*data).target_id;
            if id == 0 {
                None
            } else {
                Some(Self {
                    id,
                    agent_type: (*data).target_type.try_into(),
                    species_id: (*data).target_species_id,
                    attitude: (*data).target_attitude.try_into(),
                    health: (*data).target_health,
                    barrier: (*data).target_barrier,
                    breakbar_state: (*data).target_breakbar_state.try_into(),
                    breakbar: (*data).target_breakbar,
                })
            }
        }
    }
}
