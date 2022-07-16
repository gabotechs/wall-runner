use crate::levels::{LevelSection, LevelStructure};
use bevy::prelude::*;

pub fn level() -> LevelStructure {
    LevelStructure {
        sections: vec![
            LevelSection {
                blocks: vec![
                    shape::Box {
                        min_x: 0.0,
                        max_x: 5.0,
                        min_y: 0.0,
                        max_y: 1.0,
                        min_z: 0.0,
                        max_z: 40.0,
                    },
                    shape::Box {
                        min_x: 0.0,
                        max_x: 5.0,
                        min_y: 2.5,
                        max_y: 3.0,
                        min_z: 20.0,
                        max_z: 30.0,
                    },
                ],
                ..default()
            },
            LevelSection {
                blocks: vec![shape::Box {
                    min_x: 0.0,
                    max_x: 1.0,
                    min_y: 3.0,
                    max_y: 6.0,
                    min_z: 0.0,
                    max_z: 20.0,
                }],
                ..default()
            },
            LevelSection {
                blocks: vec![shape::Box {
                    min_x: 4.0,
                    max_x: 5.0,
                    min_y: 6.0,
                    max_y: 9.0,
                    min_z: 0.0,
                    max_z: 20.0,
                }],
                ..default()
            },
            LevelSection {
                length: Some(10.0),
                ..default()
            },
            LevelSection {
                blocks: vec![shape::Box {
                    min_x: 0.0,
                    max_x: 5.0,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: 30.0,
                }],
                ..default()
            },
        ],
        win_z: 110.0,
    }
}
