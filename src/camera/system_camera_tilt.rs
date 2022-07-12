use super::CameraState;
use crate::camera::CameraInput;
use bevy::prelude::*;

pub fn camera_tilt(
    mut camera_state: ResMut<CameraState>,
    camera_input: Res<CameraInput>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    const TILT_ALIASING_FACTOR: f32 = 0.1;
    for mut transform in query.iter_mut() {
        let yaw_quat = Quat::from_axis_angle(Vec3::Y, camera_state.yaw);
        let pitch_quat = Quat::from_axis_angle(Vec3::X, camera_state.pitch);
        camera_state.tilt = TILT_ALIASING_FACTOR * camera_input.tilt_angle
            + (1.0 - TILT_ALIASING_FACTOR) * camera_state.tilt;
        let tilt_quat = Quat::from_axis_angle(Vec3::Z, camera_state.tilt);
        transform.rotation = yaw_quat * pitch_quat * tilt_quat;
    }
}
