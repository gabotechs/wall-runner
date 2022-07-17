use crate::{CameraInput, CameraSettings, GameCamera};
use bevy::prelude::*;

pub fn setup_camera(
    mut commands: Commands,
    camera_input: Res<CameraInput>,
    camera_settings: Res<CameraSettings>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(
                camera_input.position + camera_input.position_offset,
            )
            .with_rotation(
                Quat::from_axis_angle(Vec3::Y, camera_settings.initial_yaw)
                    * Quat::from_axis_angle(Vec3::X, camera_settings.initial_pitch),
            ),
            ..Default::default()
        })
        .insert(GameCamera);
}
