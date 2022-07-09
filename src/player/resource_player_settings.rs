use bevy::prelude::*;

pub struct PlayerSettings {
    pub height: f32,
    pub width: f32,
    pub speed: f32,
    pub jump_velocity: f32,
    pub air_control: f32,
    pub acceleration_factor: f32,
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
            width: 0.5,
            speed: 15.0,
            jump_velocity: 6.0,
            air_control: 5.0,
            acceleration_factor: 0.1,
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
            run: KeyCode::LShift,
        }
    }
}
