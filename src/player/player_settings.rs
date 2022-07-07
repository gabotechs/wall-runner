use bevy::prelude::*;

pub struct PlayerSettings {
    pub height: f32,
    pub width: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub jump_velocity: f32,
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub run: KeyCode,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings {
            height: 2.0,
            width: 1.0,
            walk_speed: 10.0,
            run_speed: 15.0,
            jump_velocity: 10.0,
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
            run: KeyCode::LShift,
        }
    }
}
