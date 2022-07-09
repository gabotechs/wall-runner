#[derive(Default)]
pub struct CameraState {
    pub pitch: f32,
    pub yaw: f32,
    pub tilt: f32,
}

impl CameraState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
