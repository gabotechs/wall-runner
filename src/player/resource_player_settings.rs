use bevy::prelude::*;

pub struct PlayerSettings {
    pub size: f32,
    pub speed: f32,
    pub jump_velocity: f32,
    pub air_control: f32,
    pub acceleration_factor: f32,
    pub initial_position: Vec3,
    pub wall_run_votes: u8,
    pub crouch_boost: f32,
    pub crouch_recharge_multiplier: f32,
    pub crouch_discharge_multiplier: f32,
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub crouch: KeyCode,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings {
            size: 1.0,
            speed: 15.0,
            jump_velocity: 6.0,
            air_control: 5.0,
            acceleration_factor: 0.1,
            initial_position: Vec3::default(),
            crouch_recharge_multiplier: 0.3,
            crouch_discharge_multiplier: 1.0,
            wall_run_votes: 4,
            crouch_boost: 2.0,
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
            crouch: KeyCode::LShift,
        }
    }
}
