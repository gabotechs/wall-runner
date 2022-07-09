use crate::player::{Player, PlayerState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn player_update_state(
    player_query: Query<(&Transform, &Velocity, &ExternalForce), With<Player>>,
    mut player_state: ResMut<PlayerState>,
) {
    for (&transform, velocity, force) in player_query.iter() {
        player_state.position = transform.translation;
        player_state.velocity = *velocity;
    }
}
