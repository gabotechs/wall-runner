use crate::Vec3;

#[derive(Default)]
pub struct CameraState {
    pub pitch: f32,
    pub yaw: f32,
    pub tilt: f32,
    pub offset: Vec3,
}

impl CameraState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
