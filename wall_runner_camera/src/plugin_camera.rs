use crate::startup_system_camera_setup::setup_camera;
use crate::system_camera_look::camera_look;
use crate::system_camera_position::camera_position;
use crate::system_camera_reset::camera_reset;
use crate::system_camera_tilt::camera_tilt;
use crate::{CameraControlEvent, CameraInput, CameraSettings, CameraState};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraState>()
            .init_resource::<CameraInput>()
            .init_resource::<CameraSettings>()
            .add_event::<CameraControlEvent>()
            .add_startup_system(setup_camera)
            .add_system(camera_tilt)
            .add_system(camera_reset)
            .add_system(camera_position)
            .add_system(camera_look);
    }
}
