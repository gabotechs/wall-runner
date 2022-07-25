use bevy::prelude::*;

#[derive(Default, Component)]
pub struct CameraInput {
    pub reset: bool,
    pub inactive: bool,
    pub position: Vec3,
    pub position_offset: Vec3,
    pub tilt_angle: f32,
}
