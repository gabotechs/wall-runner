pub mod level_genesis;
pub mod level_jump;
pub mod level_wall_run;

use bevy::prelude::*;

pub struct LevelBlock {
    pub(crate) mesh: Mesh,
    pub(crate) transform: Transform,
    pub(crate) color: Color,
}

pub struct LevelStructure {
    pub blocks: Vec<LevelBlock>,
    pub win_z: f32,
}
