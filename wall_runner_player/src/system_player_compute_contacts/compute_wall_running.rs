use crate::system_player_compute_contacts::compute_forces::PlayerForces;
use crate::{PlayerSettings, PlayerState, WallRunningState};
use bevy::prelude::*;
use std::borrow::BorrowMut;
use std::f32::consts::PI;
use wall_runner_utils::*;

pub fn compute_wall_running(
    player_forces: &PlayerForces,
    settings: &PlayerSettings,
    player_state: &mut PlayerState,
) {
    let can_be_wall_running =
        player_forces.horizontal_force.is_some() && !player_state.is_in_ground;
    if can_be_wall_running && player_state.wall_running.is_none() {
        if player_state.wall_run_vote < settings.wall_run_votes {
            player_state.wall_run_vote += settings.wall_run_up_vote;
            debug!("[+] vote wall run {}", player_state.wall_run_vote);
        }
        if player_state.wall_run_vote >= settings.wall_run_votes {
            const ANGLE_EPSILON: f32 = 0.05;
            let tangent_direction = nearest_with_angle(
                vec3_horizontal_vec2(player_forces.horizontal_force.unwrap()),
                vec3_horizontal_vec2(player_state.velocity.linvel),
                PI / 2.0 + ANGLE_EPSILON,
            );
            info!("start wall run,  direction: {:?}", tangent_direction);
            player_state.wall_running = Some(WallRunningState {
                speed: None,
                just_started: true,
                normal_force: player_forces.horizontal_force.unwrap(),
                direction: tangent_direction.normalize_or_zero(),
            });
        }
    } else if !can_be_wall_running && player_state.wall_running.is_some() {
        if player_state.wall_run_vote > 0 {
            player_state.wall_run_vote -= settings.wall_run_down_vote;
            debug!("[-] vote wall run {}", player_state.wall_run_vote);
        }
        if player_state.wall_run_vote == 0 {
            info!("stop wall run");
            player_state.wall_running = None;
        }
    } else if let Some(wall_running) = player_state.wall_running.borrow_mut() {
        wall_running.just_started = false;
    }
}
