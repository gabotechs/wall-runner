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
    mut player_input: ResMut<PlayerInput>,
    mut camera_input: ResMut<CameraInput>,
    mut level_input: ResMut<LevelInput>,
) {
    let now = time.time_since_startup().as_millis();
    if now - *last_finished < 1000 {
        return;
    }
    for ev in ev_reader.iter() {
        *last_finished = now;
        player_input.reset = true;
        camera_input.reset = true;
        if ev.win {
            *current_level += 1;
            if *current_level == LEVELS.len() {
                *current_level = 0;
            }
            level_input.name = String::from(LEVELS[*current_level]);
        }
    }
}
