use super::resource_camera_settings::*;
use super::resource_camera_state::*;
use super::startup_system_camera_setup::*;
use super::system_camera_look::*;
use super::system_camera_tilt::*;
use crate::camera::system_camera_position::camera_position;
use crate::camera::system_camera_reset::camera_reset;
use crate::camera::CameraInput;
use bevy::ecs::event::ManualEventReader;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraState>()
            .init_resource::<CameraInput>()
            .init_resource::<ManualEventReader<MouseMotion>>()
            .init_resource::<CameraSettings>()
            .add_startup_system(setup_camera)
            .add_system(camera_tilt)
            .add_system(camera_reset)
            .add_system(camera_position)
            .add_system(camera_look);
    }
}
