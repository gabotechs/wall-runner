/// Mouse sensitivity and movement speed
pub struct CameraSettings {
    pub sensitivity: f32,
    pub initial_yaw: f32,
    pub initial_pitch: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            initial_yaw: 0.0,
            initial_pitch: 0.0,
        }
    }
}
