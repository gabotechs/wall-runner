use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

pub struct WallRunningState {
    pub normal_force: Vec3,
    pub speed: f32,
    pub direction: (f32, f32),
    pub ttl_counter: u8
}

pub struct PlayerState {
    pub position: Vec3,
    pub velocity: Velocity,
    pub is_in_ground: bool,
    pub wall_running: Option<WallRunningState>,
}

impl PlayerState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            position: Vec3::new(0.0, 0.0, 0.0),
            velocity: Velocity::default(),
            is_in_ground: false,
            wall_running: None,
        }
    }
}
