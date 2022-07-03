mod camera;
mod level;
mod player;
mod scene;
mod window;

use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use rand::prelude::*;

const WORLD_SIZE: f32 = 100.0;
const GRID_SIZE: i32 = 100;

fn init_components(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: WORLD_SIZE })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    let mut rng = thread_rng();
    let x: f32 = WORLD_SIZE / (GRID_SIZE as f32);
    let z: f32 = WORLD_SIZE / (GRID_SIZE as f32);
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let random_float: f32 = rng.gen();
            let r: f32 = rng.gen();
            let g: f32 = rng.gen();
            let b: f32 = rng.gen();
            let y: f32 = random_float * 3.0;
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(x, y, z))),
                    transform: Transform::from_xyz(
                        (i as f32) * x - WORLD_SIZE / 2.0,
                        y / 2.0,
                        (j as f32) * z - WORLD_SIZE / 2.0,
                    ),
                    material: materials.add(Color::hsl(r, g, b).into()),
                    ..default()
                })
                .insert(Wireframe);
        }
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
        .insert_resource(player::PlayerPosition(-2.0, 5.0, 5.0))
        .add_plugins(DefaultPlugins)
        .add_plugin(window::WindowPlugin)
        .add_plugin(scene::ScenePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(init_components)
        .add_system(attach_camera_to_player)
        .run();
}
