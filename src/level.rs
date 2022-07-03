use bevy::prelude::*;

pub struct LevelPlugin;

pub struct LevelSection {
    blocks: Vec<shape::Box>,
}

#[derive(Component)]
pub struct LevelStructure {
    sections: Vec<LevelSection>,
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
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    min_x: block.min_x,
                    max_x: block.max_x,
                    min_y: block.min_y,
                    max_y: block.max_y,
                    min_z: current_z + block.min_z,
                    max_z: current_z + block.max_z,
                })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                ..default()
            });
        }
        current_z += max_z_in_block;
    }
}
