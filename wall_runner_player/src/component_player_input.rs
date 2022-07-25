use bevy::prelude::*;

#[derive(Default, Component)]
pub struct PlayerInput {
    pub inactive: bool,
    pub reset: bool,
    pub y_angle: f32,
}
