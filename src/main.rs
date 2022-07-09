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
                    min_y: 5.0,
                    max_y: 8.0,
                    min_z: 0.0,
                    max_z: 20.0,
                }],
                ..default()
            },
            level::LevelSection {
                length: Some(5.0),
                ..default()
            },
            level::LevelSection {
                blocks: vec![shape::Box {
                    min_x: 0.0,
                    max_x: 5.0,
                    min_y: 9.0,
                    max_y: 10.0,
                    min_z: 0.0,
                    max_z: 30.0,
                }],
                ..default()
            },
        ],
    }
}

fn attach_camera_to_player(
    mut camera_state: ResMut<camera::CameraState>,
    player_settings: Res<player::PlayerSettings>,
    mut player_state: ResMut<player::PlayerState>,
) {
    const TILT_ANGLE_FACTOR: f32 = 0.4;
    camera_state.position = player_state.position;
    if let Some(wall_running) = &player_state.wall_running {
        let wall_vector = Vec2::new(wall_running.normal_force.x, wall_running.normal_force.z);
        let move_vector = Vec2::new(
            player_state.velocity.linvel.x,
            player_state.velocity.linvel.z,
        );
        let angle = wall_vector.angle_between(move_vector);
        if !angle.is_nan() {
            camera_state.tilt_target =
                angle.sin() * move_vector.length() / player_settings.speed * TILT_ANGLE_FACTOR;
        }
    } else {
        camera_state.tilt_target = 0.0;
    }
    player_state.y_angle = camera_state.yaw;
}

const INITIAL_POS: (f32, f32, f32) = (2.5, 3.0, -2.0);
const FAIL_Y: f32 = -1.0;

fn reset_player_if_fall(
    mut player_state: ResMut<player::PlayerState>,
    mut player_query: Query<
        (&mut Transform, &mut Velocity, &mut ExternalForce),
        With<player::Player>,
    >,
    mut camera_state: ResMut<camera::CameraState>,
) {
    if player_state.position.y < FAIL_Y {
        for (mut transform, mut velocity, mut force) in player_query.iter_mut() {
            transform.translation = Vec3::from(INITIAL_POS);
            velocity.linvel = Vec3::default();
            force.force = Vec3::default();
        }
        player_state.reset();
        camera_state.reset();
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
        .insert_resource(player::PlayerState {
            position: Vec3::from(INITIAL_POS),
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
        .add_system(cursor_grab)
        .add_system(reset_player_if_fall)
        .add_startup_system(initial_grab_cursor)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_system_to_stage(CoreStage::PreUpdate, attach_camera_to_player)
        .run();
}
