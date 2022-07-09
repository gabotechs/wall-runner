use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct LevelPlugin;

pub struct LevelSection {
    pub(crate) blocks: Vec<shape::Box>,
}

#[derive(Component)]
pub struct LevelStructure {
    pub(crate) sections: Vec<LevelSection>,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_level);
    }
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
                    transform: Transform::from_xyz(0.0, 0.0, current_z - block.max_z),
                    material: materials.add(Color::rgb(0.2, 0.3, 0.8).into()),
                    ..default()
                })
                .insert(RigidBody::Fixed)
                .insert(collider.unwrap());
        }
        current_z -= max_z_in_block;
    }
}
