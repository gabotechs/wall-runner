use super::entity_player::Player;
use super::resource_player_state::PlayerState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn update_gravity(
    player_state: Res<PlayerState>,
    mut player_query: Query<&mut GravityScale, With<Player>>,
) {
    for mut gravity in player_query.iter_mut() {
        if player_state.wall_running.is_some() {
            gravity.0 = 0.3;
        } else {
            gravity.0 = 1.0;
        }
    }
}
