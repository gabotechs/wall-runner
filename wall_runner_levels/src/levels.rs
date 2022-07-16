pub mod level_genesis;
pub mod level_jump;
use bevy::prelude::*;

#[derive(Default)]
pub struct LevelSection {
    pub length: Option<f32>,
    pub blocks: Vec<shape::Box>,
}

pub struct LevelStructure {
    pub sections: Vec<LevelSection>,
    pub win_z: f32,
}
