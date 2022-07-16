use bevy::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct CameraControlEvent {
    pub look: Vec2,
}
