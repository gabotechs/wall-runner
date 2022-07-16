use bevy::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct InputEvent {
    pub movement: Vec2,
    pub look: Vec2,
    pub is_crouching: bool,
    pub jump: bool,
}
