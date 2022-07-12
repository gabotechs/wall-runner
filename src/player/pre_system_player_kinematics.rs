use bevy::prelude::*;

#[derive(Copy, Clone, Default)]
pub struct PlayerKinematics {
    pub displacement: Vec2,
    pub vertical_impulse: f32,
    pub force: Vec3,
    pub gravity: f32,
}

pub fn player_reset_kinematics(mut kinematics: ResMut<PlayerKinematics>) {
    *kinematics = PlayerKinematics::default();
}
