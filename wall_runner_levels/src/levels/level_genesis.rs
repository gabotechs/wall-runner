use crate::levels::{LevelBlock, LevelStructure};
use bevy::prelude::*;

pub fn level() -> LevelStructure {
    LevelStructure {
        blocks: vec![
            LevelBlock {
                mesh: Mesh::from(shape::Box {
                    min_x: 0.0,
                    max_x: 5.0,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: 40.0,
                }),
                transform: Transform::from_xyz(0.0, 0.0, -40.0),
                color: Color::rgb(0.2, 0.3, 0.8),
            },
            LevelBlock {
                mesh: Mesh::from(shape::Box {
                    min_x: 0.0,
                    max_x: 5.0,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: 10.0,
                }),
                transform: Transform::from_xyz(0.0, 2.5, -25.0),
                color: Color::rgb(0.2, 0.3, 0.8),
            },
            LevelBlock {
                mesh: Mesh::from(shape::Box {
                    min_x: 0.0,
                    max_x: 1.0,
                    min_y: 0.0,
                    max_y: 3.0,
                    min_z: 0.0,
                    max_z: 20.0,
                }),
                transform: Transform::from_xyz(0.0, 3.0, -55.0),
                color: Color::rgb(0.2, 0.3, 0.8),
            },
            LevelBlock {
                mesh: Mesh::from(shape::Box {
                    min_x: 0.0,
                    max_x: 1.0,
                    min_y: 0.0,
                    max_y: 3.0,
                    min_z: 0.0,
                    max_z: 20.0,
                }),
                transform: Transform::from_xyz(4.0, 6.0, -75.0),
                color: Color::rgb(0.2, 0.3, 0.8),
            },
            LevelBlock {
                mesh: Mesh::from(shape::Box {
                    min_x: 0.0,
                    max_x: 5.0,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: 30.0,
                }),
                transform: Transform::from_xyz(0.0, 0.0, -115.0),
                color: Color::rgb(0.2, 0.3, 0.8),
            },
        ],
        win_z: 100.0,
    }
}
