use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct PlayerKinematics {
    pub displacement: Vec2,
    pub vertical_impulse: f32,
    pub force: Vec3,
    pub gravity: f32,
}

impl Default for PlayerKinematics {
    fn default() -> Self {
        PlayerKinematics {
            displacement: Vec2::default(),
            vertical_impulse: 0.0,
            force: Vec3::default(),
            gravity: 1.0,
        }
    }
}
