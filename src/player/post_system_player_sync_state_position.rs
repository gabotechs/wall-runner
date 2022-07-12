use crate::player::{Player, PlayerState};
use bevy::prelude::*;

pub fn player_sync_state_position(
    player_query: Query<&Transform, With<Player>>,
    mut player_state: ResMut<PlayerState>,
) {
    for &transform in player_query.iter() {
        player_state.position = transform.translation;
    }
}
