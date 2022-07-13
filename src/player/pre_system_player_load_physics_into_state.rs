use crate::player::{Player, PlayerState};
use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;

pub fn player_load_physics_into_state(
    player_query: Query<(&Transform, &Velocity), With<Player>>,
    mut player_state: ResMut<PlayerState>,
) {
    for (&transform, &velocity) in player_query.iter() {
        player_state.kinematics.displacement.x = velocity.linvel.x;
        player_state.kinematics.displacement.y = velocity.linvel.z;
        player_state.kinematics.vertical_impulse = velocity.linvel.y;
        player_state.position = transform.translation;
    }
}
