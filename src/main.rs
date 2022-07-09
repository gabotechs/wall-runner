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
            },
            level::LevelSection {
                blocks: vec![
                    shape::Box {
                        min_x: 0.0,
                        max_x: 1.0,
                        min_y: 2.0,
                        max_y: 5.0,
                        min_z: 0.0,
                        max_z: 20.0,
                    },
                    shape::Box {
                        min_x: 4.0,
                        max_x: 5.0,
                        min_y: 2.0,
                        max_y: 5.0,
                        min_z: 0.0,
                        max_z: 20.0,
                    },
                ],
            },
            level::LevelSection {
                blocks: vec![shape::Box {
                    min_x: 0.0,
                    max_x: 5.0,
                    min_y: 4.0,
                    max_y: 5.0,
                    min_z: 0.0,
                    max_z: 30.0,
                }],
            },
        ],
    }
}

fn attach_camera_to_player(
    mut camera_state: ResMut<camera::CameraState>,
    mut player_state: ResMut<player::PlayerState>,
) {
    camera_state.position = player_state.position;
    if let Some(wall_running) = &player_state.wall_running {
        let wall_vector = Vec2::new(wall_running.normal_force.x, wall_running.normal_force.z);
        let move_vector = Vec2::new(
            player_state.velocity.linvel.x,
            player_state.velocity.linvel.z,
        );
        let angle = wall_vector.angle_between(move_vector);
        camera_state.tilt_target = angle.sin() * 0.3;
    } else {
        camera_state.tilt_target = 0.0;
    }
    player_state.y_angle = camera_state.yaw;
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
            y_angle: std::f32::consts::PI,
            position: Vec3::new(2.5, 3.0, 2.0),
            ..default()
        })
        .insert_resource(camera::CameraState {
            yaw: std::f32::consts::PI,
            pitch: 0.0,
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
        .add_startup_system(initial_grab_cursor)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_system_to_stage(CoreStage::PreUpdate, attach_camera_to_player)
        .run();
}
