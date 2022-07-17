use crate::system_gamepad_connections::gamepad_connections;
use crate::system_gamepad_input::gamepad_input;
use crate::system_keyboard_input::keyboard_input;
use bevy::ecs::event::ManualEventReader;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ManualEventReader<MouseMotion>>()
            .add_system_to_stage(CoreStage::PreUpdate, keyboard_input)
            .add_system_to_stage(CoreStage::PreUpdate, gamepad_input)
            .add_system(gamepad_connections);
    }
}
