use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_kira_audio::*;
use bevy_rapier3d::prelude::*;

use wall_runner_camera::*;
use wall_runner_input::*;
use wall_runner_player::*;

mod camera_player_sync;
mod level;
mod scene;
mod window;

fn get_levels() -> level::LevelStructure {
    level::LevelStructure {
        sections: vec![
            level::LevelSection {
                blocks: vec![
                    shape::Box {
                        min_x: 0.0,
                        max_x: 5.0,
                        min_y: 0.0,
                        max_y: 1.0,
                        min_z: 0.0,
                        max_z: 40.0,
                    },
                    shape::Box {
                        min_x: 0.0,
                        max_x: 5.0,
                        min_y: 2.5,
                        max_y: 3.0,
                        min_z: 20.0,
                        max_z: 30.0,
                    },
                ],
                ..default()
            },
            level::LevelSection {
                blocks: vec![shape::Box {
                    min_x: 0.0,
                    max_x: 1.0,
                    min_y: 3.0,
                    max_y: 6.0,
                    min_z: 0.0,
                    max_z: 20.0,
                }],
                ..default()
            },
            level::LevelSection {
                blocks: vec![shape::Box {
                    min_x: 4.0,
                    max_x: 5.0,
                    min_y: 6.0,
                    max_y: 9.0,
                    min_z: 0.0,
                    max_z: 20.0,
                }],
                ..default()
            },
            level::LevelSection {
                length: Some(10.0),
                ..default()
            },
            level::LevelSection {
                blocks: vec![shape::Box {
                    min_x: 0.0,
                    max_x: 5.0,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: 30.0,
                }],
                ..default()
            },
        ],
    }
}

const INITIAL_POS: (f32, f32, f32) = (2.5, 3.0, -2.0);
const FAIL_Y: f32 = -1.0;

fn reset_player_if_fall(
    player_state: ResMut<PlayerState>,
    mut player_input: ResMut<PlayerInput>,
    mut camera_input: ResMut<CameraInput>,
) {
    if player_state.position.y < FAIL_Y {
        player_input.reset = true;
        camera_input.reset = true;
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
        .insert_resource(PlayerSettings {
            initial_position: Vec3::from(INITIAL_POS),
            ..default()
        })
        .insert_resource(get_levels())
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(window::WindowPlugin)
        .add_plugin(scene::ScenePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(level::LevelPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(cursor_grab)
        .add_system(reset_player_if_fall)
        .add_startup_system(initial_grab_cursor)
        .add_system_to_stage(
            CoreStage::PreUpdate,
            camera_player_sync::attach_camera_to_player,
        )
        .run();
}
