use crate::{CameraInput, CameraSettings, CameraState, GameCamera};
use bevy::prelude::*;

pub fn camera_reset(
    camera_settings: Res<CameraSettings>,
    mut camera_input: ResMut<CameraInput>,
    mut camera_state: ResMut<CameraState>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
) {
    if camera_input.reset {
        info!("resetting wall_runner_camera");
        for mut transform in camera_query.iter_mut() {
            transform.rotation = Quat::from_axis_angle(Vec3::Y, camera_settings.initial_yaw)
                * Quat::from_axis_angle(Vec3::X, camera_settings.initial_pitch);
            transform.translation = camera_input.position;
        }
        camera_state.reset();
        camera_input.reset = false;
    }
}
