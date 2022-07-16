use bevy::prelude::*;
use bevy_atmosphere::{AtmosphereMat, AtmospherePlugin};
use bevy_rapier3d::prelude::*;

pub struct LevelPlugin;

#[derive(Default)]
pub struct LevelSection {
    pub length: Option<f32>,
    pub blocks: Vec<shape::Box>,
}

#[derive(Component)]
pub struct LevelStructure {
    pub sections: Vec<LevelSection>,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AtmosphereMat {
            planet_radius: AtmosphereMat::default().planet_radius - 5000.0,
            ..default()
        })
        .add_plugin(AtmospherePlugin::default())
        .add_startup_system(spawn_environment)
        .add_startup_system(spawn_level);
    }
}

fn spawn_environment(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 1.0,
        ..default()
    });
}

fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    level_structure_query: Res<LevelStructure>,
) {
    let mut current_z = 0.0;
    for section in level_structure_query.sections.iter() {
        let mut max_z_in_block = 0.0;
        for block in section.blocks.iter() {
            if block.max_z > max_z_in_block {
                max_z_in_block = block.max_z;
            }
            let mesh = Mesh::from(shape::Box {
                min_x: block.min_x,
                max_x: block.max_x,
                min_y: block.min_y,
                max_y: block.max_y,
                min_z: block.min_z,
                max_z: block.max_z,
            });
            let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh);
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(mesh),
                    transform: Transform::from_xyz(0.0, 0.0, current_z - block.max_z - block.min_z),
                    material: materials.add(Color::rgb(0.2, 0.3, 0.8).into()),
                    ..default()
                })
                .insert(RigidBody::Fixed)
                .insert(collider.unwrap());
        }
        if let Some(section_length) = section.length {
            current_z -= section_length;
        } else {
            current_z -= max_z_in_block;
        }
    }
}
