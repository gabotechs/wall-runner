use crate::player::pre_system_player_kinematics::PlayerKinematics;
use bevy::prelude::*;

pub struct WallRunningState {
    pub just_started: bool,
    pub normal_force: Vec3,
    pub direction: Vec2,
    pub speed: Option<f32>,
}

pub struct CrouchState {
    pub is_crouching: bool,
    pub charge: f32,
}

pub struct PlayerState {
    pub kinematics: PlayerKinematics,
    pub position: Vec3,
    pub is_in_ground: bool,
    pub wall_run_vote: u8,
    pub wall_running: Option<WallRunningState>,
    pub crouch_state: CrouchState,
    pub head_offset: f32,
}

impl PlayerState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            head_offset: 1.0,
            position: Vec3::default(),
            kinematics: PlayerKinematics::default(),
            is_in_ground: false,
            wall_run_vote: 0,
            wall_running: None,
            crouch_state: CrouchState {
                is_crouching: false,
                charge: 1.0,
            },
        }
    }
}
