use bevy::prelude::*;

pub struct PlayerSettings {
    pub size: f32,
    pub speed: f32,
    pub jump_velocity: f32,
    pub air_control: f32,
    pub acceleration_factor: f32,
    pub initial_position: Vec3,
    pub wall_run_gravity: f32,
    pub wall_run_votes: u8,
    pub wall_run_up_vote: u8,
    pub wall_run_down_vote: u8,
    pub ground_votes: u8,
    pub ground_up_vote: u8,
    pub ground_down_vote: u8,
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
            air_control: 3.0,
            acceleration_factor: 0.1,
            initial_position: Vec3::default(),
            crouch_recharge_multiplier: 0.3,
            crouch_discharge_multiplier: 1.0,
            ground_votes: 20,
            ground_up_vote: 20,
            ground_down_vote: 1,
            wall_run_gravity: 0.3,
            wall_run_votes: 2,
            wall_run_up_vote: 1,
            wall_run_down_vote: 1,
            crouch_boost: 10.0,
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
            crouch: KeyCode::LShift,
        }
    }
}
