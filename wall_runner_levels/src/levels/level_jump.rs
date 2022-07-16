use crate::levels::{LevelSection, LevelStructure};
use bevy::prelude::*;

pub fn level() -> LevelStructure {
    LevelStructure {
        sections: vec![
            LevelSection {
                blocks: vec![shape::Box {
                    min_x: 2.0,
                    max_x: 3.0,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: 30.0,
                }],
                ..default()
            },
            LevelSection {
                length: Some(20.0),
                ..default()
            },
            LevelSection {
                blocks: vec![shape::Box {
                    min_x: 2.0,
                    max_x: 3.0,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: 30.0,
                }],
                ..default()
            },
        ],
        win_z: 70.0,
    }
}
