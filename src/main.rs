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
    player_state.y_angle = camera_state.yaw;
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
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_system_to_stage(CoreStage::PreUpdate, attach_camera_to_player)
        .run();
}
