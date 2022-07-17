use crate::levels::{LevelBlock, LevelStructure};
use bevy::prelude::*;

pub fn level() -> LevelStructure {
    LevelStructure {
        blocks: vec![
            LevelBlock {
                mesh: Mesh::from(shape::Box {
                    min_x: 0.0,
                    max_x: 1.0,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: 30.0,
                }),
                transform: Transform::from_xyz(2.0, 0.0, -30.0),
                color: Color::rgb(0.2, 0.3, 0.8),
            },
            LevelBlock {
                mesh: Mesh::from(shape::Box {
                    min_x: 2.0,
                    max_x: 3.0,
                    min_y: 0.0,
                    max_y: 3.0,
                    min_z: 0.0,
                    max_z: 40.0,
                }),
                transform: Transform::from_xyz(3.0, 2.5, -70.0),
                color: Color::rgb(0.2, 0.3, 0.8),
            },
            LevelBlock {
                mesh: Mesh::from(shape::Box {
                    min_x: 0.0,
                    max_x: 1.0,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: 30.0,
                }),
                transform: Transform::from_xyz(2.0, 0.0, -107.0),
                color: Color::rgb(0.2, 0.3, 0.8),
            },
        ],
        win_z: 100.0,
    }
}
