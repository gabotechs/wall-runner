extern crate core;

mod level;
mod level_genesis;

pub use level::*;

pub fn level(n: &str) -> LevelStructure {
    match n {
        "genesis" => level_genesis::level(),
        _ => panic!("Level {} does not exist", n),
    }
}
