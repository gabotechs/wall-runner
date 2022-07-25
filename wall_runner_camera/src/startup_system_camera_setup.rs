use crate::{CameraInput, CameraSettings, CameraState};
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands, camera_settings: Res<CameraSettings>) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_rotation(
                Quat::from_axis_angle(Vec3::Y, camera_settings.initial_yaw)
                    * Quat::from_axis_angle(Vec3::X, camera_settings.initial_pitch),
            ),
            ..Default::default()
        })
        .insert(CameraState::default())
        .insert(CameraInput::default());
}
