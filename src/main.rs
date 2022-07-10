extern crate core;

mod camera;
mod level;
mod player;
mod scene;
mod window;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn get_levels() -> level::LevelStructure {
    level::LevelStructure {
        sections: vec![
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
            level::LevelSection {
                blocks: vec![shape::Box {
                    min_x: 0.0,
                    max_x: 1.0,
                    min_y: 2.0,
                    max_y: 5.0,
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

fn attach_camera_to_player(
    camera_state: ResMut<camera::CameraState>,
    player_state: Res<player::PlayerState>,
    mut camera_input: ResMut<camera::CameraInput>,
    mut player_input: ResMut<player::PlayerInput>,
    player_settings: Res<player::PlayerSettings>,
) {
    const TILT_ANGLE_FACTOR: f32 = 0.4;
    const HEAD_OFFSET: f32 = 0.1;
    camera_input.position = player_state.position;
    camera_input.position.y += HEAD_OFFSET;
    if let Some(wall_running) = &player_state.wall_running {
        let wall_vector = Vec2::new(wall_running.normal_force.x, wall_running.normal_force.z);
        let move_vector = Vec2::new(
            player_state.velocity.linvel.x,
            player_state.velocity.linvel.z,
        );
        let angle = wall_vector.angle_between(move_vector);
        if !angle.is_nan() {
            camera_input.tilt_angle =
                angle.sin() * move_vector.length() / player_settings.speed * TILT_ANGLE_FACTOR;
        }
    } else {
        camera_input.tilt_angle = 0.0;
    }
    player_input.y_angle = camera_state.yaw;
}

const INITIAL_POS: (f32, f32, f32) = (2.5, 3.0, -2.0);
const FAIL_Y: f32 = -1.0;

fn reset_player_if_fall(
    player_state: ResMut<player::PlayerState>,
    mut player_input: ResMut<player::PlayerInput>,
    mut camera_input: ResMut<camera::CameraInput>,
) {
    if player_state.position.y < FAIL_Y {
        player_input.reset = true;
        camera_input.reset = true;
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(player::PlayerSettings {
            initial_position: Vec3::from(INITIAL_POS),
            ..default()
        })
        .insert_resource(get_levels())
        .add_plugins(DefaultPlugins)
        .add_plugin(window::WindowPlugin)
        .add_plugin(scene::ScenePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(level::LevelPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(cursor_grab)
        .add_system(reset_player_if_fall)
        .add_startup_system(initial_grab_cursor)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_system_to_stage(CoreStage::PreUpdate, attach_camera_to_player)
        .run();
}
