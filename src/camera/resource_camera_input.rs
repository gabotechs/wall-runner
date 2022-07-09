use crate::Vec3;

#[derive(Default)]
pub struct CameraInput {
    pub reset: bool,
    pub position: Vec3,
    pub tilt_angle: f32,
}