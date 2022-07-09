use bevy::prelude::*;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
pub struct CameraState {
    pub pitch: f32,
    pub yaw: f32,
    pub tilt_target: f32,
    pub tilt: f32,
    pub position: Vec3,
}

impl CameraState {
    pub fn reset(&mut self) {
        let new = Self::default();
        self.pitch = new.pitch;
        self.yaw = new.yaw;
        self.tilt_target = new.tilt_target;
        self.tilt = new.tilt;
        self.position = new.position;
    }
}
