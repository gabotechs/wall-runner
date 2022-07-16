use crate::component_level::LevelComponent;
use crate::levels::level_genesis;
use crate::levels::level_jump;
use crate::levels::LevelStructure;
use crate::resource_level_status::LevelStatus;
use crate::LevelInput;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn level(n: &str) -> LevelStructure {
    match n {
        "jump" => level_jump::level(),
        "genesis" => level_genesis::level(),
        _ => panic!("Level {} does not exist", n),
    }
}

pub fn choose_level(
    mut level_status: ResMut<LevelStatus>,
    level_name: Res<LevelInput>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    level_component_query: Query<Entity, With<LevelComponent>>,
) {
    if level_status.current_level == level_name.name {
        return;
    }

    for component in level_component_query.iter() {
        commands.entity(component).despawn();
    }

    let mut current_z = 0.0;
    let level_structure = level(level_name.name.as_str());
    level_status.current_level = level_name.name.clone();
    level_status.current_win_z = level_structure.win_z;
    for section in level_structure.sections.iter() {
        let mut max_z_in_block = 0.0;
        for block in section.blocks.iter() {
            if block.max_z > max_z_in_block {
                max_z_in_block = block.max_z;
            }
            let mesh = Mesh::from(*block);
            let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh);
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(mesh),
                    transform: Transform::from_xyz(0.0, 0.0, current_z - block.max_z - block.min_z),
                    material: materials.add(Color::rgb(0.2, 0.3, 0.8).into()),
                    ..default()
                })
                .insert(RigidBody::Fixed)
                .insert(collider.unwrap())
                .insert(LevelComponent);
        }
        if let Some(section_length) = section.length {
            current_z -= section_length;
        } else {
            current_z -= max_z_in_block;
        }
    }
}
