use crate::{CameraInput, CameraSettings, CameraState, GameCamera};
use bevy::prelude::*;

pub fn camera_reset(
    camera_settings: Res<CameraSettings>,
    mut camera_query: Query<(&mut Transform, &mut CameraInput, &mut CameraState), With<GameCamera>>,
) {
    for (mut transform, mut camera_input, mut camera_state) in camera_query.iter_mut() {
        if camera_input.reset {
            info!("resetting wall_runner_camera");
            transform.rotation = Quat::from_axis_angle(Vec3::Y, camera_settings.initial_yaw)
                * Quat::from_axis_angle(Vec3::X, camera_settings.initial_pitch);
            transform.translation = camera_input.position;
            camera_state.reset();
            camera_input.reset = false;
        }
    }
}
