use crate::component_level::LevelComponent;
use crate::levels::LevelStructure;
use crate::levels::{level_genesis, level_jump, level_wall_run};
use crate::resource_level_status::LevelStatus;
use crate::LevelInput;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn level(n: &str) -> LevelStructure {
    match n {
        "jump" => level_jump::level(),
        "genesis" => level_genesis::level(),
        "wall_run" => level_wall_run::level(),
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

    let level_structure = level(level_name.name.as_str());
    level_status.current_level = level_name.name.clone();
    level_status.current_win_z = level_structure.win_z;
    for block in level_structure.blocks.iter() {
        let collider = Collider::from_bevy_mesh(&block.mesh, &ComputedColliderShape::TriMesh);
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(block.mesh.clone()),
                transform: block.transform,
                material: materials.add(block.color.into()),
                ..default()
            })
            .insert(RigidBody::Fixed)
            .insert(collider.unwrap())
            .insert(LevelComponent);
    }
}
