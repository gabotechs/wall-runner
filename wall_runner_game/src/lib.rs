use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_kira_audio::*;
use bevy_rapier3d::prelude::*;

use wall_runner_camera::*;
use wall_runner_input::*;
use wall_runner_levels::*;
use wall_runner_player::*;

mod camera_player_sync;
mod level_player_sync;
mod window;

const LEVELS: [&str; 2] = ["jump", "genesis"];

const INITIAL_POS: (f32, f32, f32) = (2.5, 3.0, -2.0);

fn player_level_finish(
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

fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut camera_input: ResMut<CameraInput>,
) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        camera_input.inactive = !camera_input.inactive;
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

pub fn app() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(PlayerSettings {
            initial_position: Vec3::from(INITIAL_POS),
            ..default()
        })
        .insert_resource(LevelInput {
            name: String::from(LEVELS[0]),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(window::WindowPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(cursor_grab)
        .add_startup_system(initial_grab_cursor)
        .add_system_to_stage(CoreStage::PostUpdate, player_level_finish)
        .add_system_to_stage(
            CoreStage::PreUpdate,
            level_player_sync::attach_player_to_level,
        )
        .add_system_to_stage(
            CoreStage::PreUpdate,
            camera_player_sync::attach_camera_to_player,
        )
        .run();
}
