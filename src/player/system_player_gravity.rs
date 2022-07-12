use super::resource_player_state::PlayerState;
use crate::player::pre_system_player_kinematics::PlayerKinematics;
use bevy::prelude::*;

pub fn update_gravity(player_state: Res<PlayerState>, mut kinematics: ResMut<PlayerKinematics>) {
    if player_state.wall_running.is_some() {
        kinematics.gravity = 0.3;
    } else {
        kinematics.gravity = 1.0;
    }
}
