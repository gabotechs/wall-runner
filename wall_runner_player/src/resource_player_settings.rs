use bevy::prelude::*;

pub struct PlayerSettings {
    pub height: f32,
    pub height_crouching: f32,
    pub width: f32,
    pub run_speed: f32,
    pub jump_speed: f32,
    pub air_control: f32,
    pub acceleration_factor: f32,
    pub initial_position: Vec3,
    pub wall_run_gravity: f32,
    pub wall_run_votes: i16,
    pub wall_run_up_vote: i16,
    pub wall_run_down_vote: i16,
    pub ground_votes: i16,
    pub ground_up_vote: i16,
    pub ground_down_vote: i16,
    pub crouch_boost: f32,
    pub crouch_recharge_multiplier: f32,
    pub crouch_discharge_multiplier: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings {
            height: 2.0,
            height_crouching: 1.0,
            width: 0.3,
            run_speed: 15.0,
            jump_speed: 6.0,
            air_control: 0.5,
            acceleration_factor: 0.1,
            initial_position: Vec3::default(),
            crouch_recharge_multiplier: 0.3,
            crouch_discharge_multiplier: 1.0,
            ground_votes: 20,
            ground_up_vote: 20,
            ground_down_vote: 1,
            wall_run_gravity: 0.3,
            wall_run_votes: 4,
            wall_run_up_vote: 4,
            wall_run_down_vote: 1,
            crouch_boost: 10.0,
        }
    }
}
