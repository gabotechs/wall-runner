use crate::player::pre_system_player_init_kinematics::PlayerKinematics;
use crate::player::{Player, PlayerState};
use bevy::prelude::*;
use bevy_rapier3d::dynamics::GravityScale;
use bevy_rapier3d::prelude::{ExternalForce, Velocity};

pub fn player_dump_kinematics(
    kinematics: Res<PlayerKinematics>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<(&mut Velocity, &mut ExternalForce, &mut GravityScale), With<Player>>,
) {
    for (mut velocity, mut force, mut gravity) in player_query.iter_mut() {
        velocity.linvel.y += kinematics.vertical_impulse;
        if kinematics.force.length() > 0.0 {
            force.force = kinematics.force;
        } else {
            velocity.linvel.x = kinematics.displacement.x;
            velocity.linvel.z = kinematics.displacement.y;
        }
        gravity.0 = kinematics.gravity;

        player_state.kinematics.displacement.x = velocity.linvel.x;
        player_state.kinematics.displacement.y = velocity.linvel.z;
        player_state.kinematics.vertical_impulse = velocity.linvel.y;
        player_state.kinematics.force = force.force;
        player_state.kinematics.gravity = gravity.0;
    }
}
