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
                blocks: vec![shape::Box {
                    min_x: 0.0,
                    max_x: 1.0,
                    min_y: 2.0,
                    max_y: 5.0,
                    min_z: 0.0,
                    max_z: 20.0,
                }],
            },
        ],
    }
}

fn attach_camera_to_player(
    mut camera_pos: ResMut<camera::CameraPosition>,
    camera_state: Res<camera::CameraState>,
    mut player_angle: ResMut<player::DisplacementAngle>,
    player_pos: Res<player::PlayerPosition>,
) {
    camera_pos.0 = player_pos.0;
    camera_pos.1 = player_pos.1;
    camera_pos.2 = player_pos.2;
    player_angle.0 = camera_state.yaw;
}

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(player::PlayerPosition(2.5, 4.0, 2.0))
        .insert_resource(camera::CameraState {
            yaw: std::f32::consts::PI,
            pitch: 0.0,
        })
        .insert_resource(get_levels())
        .add_plugins(DefaultPlugins)
        .add_plugin(window::WindowPlugin)
        .add_plugin(scene::ScenePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(level::LevelPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(attach_camera_to_player)
        .run();
}
