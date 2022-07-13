use crate::player::resource_player_kinematics::PlayerKinematics;
use bevy::prelude::*;

pub fn player_reset_kinematics(mut kinematics: ResMut<PlayerKinematics>) {
    *kinematics = PlayerKinematics::default();
}
