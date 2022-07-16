use bevy::prelude::*;
use wall_runner_levels::*;
use wall_runner_player::*;

pub fn attach_player_to_level(
    player_state: Res<PlayerState>,
    mut level_input: ResMut<LevelPlayerPositionInput>,
) {
    level_input.position = player_state.position;
}
