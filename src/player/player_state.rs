use bevy::prelude::*;

pub struct PlayerState {
    pub position: Vec3,
    pub y_angle: f32,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            position: Vec3::new(0.0, 0.0, 0.0),
            y_angle: 0.0,
        }
    }
}
