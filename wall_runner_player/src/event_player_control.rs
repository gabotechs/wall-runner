use bevy::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct PlayerControlEvent {
    pub movement: Vec2,
    pub is_crouching: bool,
    pub jump: bool,
}
