use bevy::prelude::*;

pub struct WallRunningState {
    pub start_ms: u128,
    pub normal_force: Vec3
}

pub struct PlayerState {
    pub position: Vec3,
    pub y_angle: f32,
    pub is_in_ground: bool,
    pub inertia: Vec3,
    pub wall_running: Option<WallRunningState>
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            position: Vec3::new(0.0, 0.0, 0.0),
            inertia: Vec3::new(0.0, 0.0, 0.0),
            y_angle: 0.0,
            is_in_ground: false,
            wall_running: None
        }
    }
}
