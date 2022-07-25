use crate::LEVELS;
use bevy::prelude::*;
use wall_runner_camera::CameraInput;
use wall_runner_levels::{EventLevelFinish, LevelInput};
use wall_runner_player::PlayerInput;

pub fn player_level_finish(
    mut last_finished: Local<u128>,
    time: Res<Time>,
    mut current_level: Local<usize>,
    mut ev_reader: EventReader<EventLevelFinish>,
    mut level_input: ResMut<LevelInput>,
    mut camera_query: Query<&mut CameraInput>,
    mut player_query: Query<&mut PlayerInput>,
) {
    let now = time.time_since_startup().as_millis();
    if now - *last_finished < 1000 {
        return;
    }
    for ev in ev_reader.iter() {
        // todo: this assumes one player only
        for mut camera_input in camera_query.iter_mut() {
            camera_input.reset = true;
        }
        for mut player_input in player_query.iter_mut() {
            player_input.reset = true;
        }
        *last_finished = now;
        if ev.win {
            *current_level += 1;
            if *current_level == LEVELS.len() {
                *current_level = 0;
            }
            level_input.name = String::from(LEVELS[*current_level]);
        }
    }
}
