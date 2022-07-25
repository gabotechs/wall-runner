use bevy::prelude::*;
use wall_runner_levels::*;
use wall_runner_player::*;

pub fn attach_player_to_level(
    mut level_input: ResMut<LevelPlayerPositionInput>,
    mut player_query: Query<&mut PlayerState, With<Player>>,
) {
    for player_state in player_query.iter_mut() {
        // todo: this assumes only one player
        level_input.position = player_state.position;
    }
}
