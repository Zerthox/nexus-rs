use super::RealTimeData;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CameraData {
    /// Camera position in the game world.
    pub camera_position: [f32; 3],

    /// Camera facing direction.
    pub camera_facing: [f32; 3],

    /// Camera field of view.
    pub camera_fov: f32,

    /// Whether action camera is enabled.
    pub is_action_camera: bool,
}

impl CameraData {
    pub unsafe fn read(data: *const RealTimeData) -> Self {
        Self {
            camera_position: (*data).camera_position,
            camera_facing: (*data).camera_facing,
            camera_fov: (*data).camera_fov,
            is_action_camera: (*data).is_action_camera.is_action_camera(),
        }
    }
}
