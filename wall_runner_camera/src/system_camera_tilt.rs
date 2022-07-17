use crate::{CameraInput, CameraState};
use bevy::prelude::*;

const STEP: f32 = 2.0;

pub fn camera_tilt(
    camera_input: Res<CameraInput>,
    time: Res<Time>,
    mut camera_state: ResMut<CameraState>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let step = STEP * time.delta().as_secs_f32();
    let diff_tilt = camera_input.tilt_angle - camera_state.tilt;
    let diff_tilt = diff_tilt.clamp(-step, step);
    camera_state.tilt += diff_tilt;
    for mut transform in query.iter_mut() {
        let yaw_quat = Quat::from_axis_angle(Vec3::Y, camera_state.yaw);
        let pitch_quat = Quat::from_axis_angle(Vec3::X, camera_state.pitch);
        let tilt_quat = Quat::from_axis_angle(Vec3::Z, camera_state.tilt);
        transform.rotation = yaw_quat * pitch_quat * tilt_quat;
    }
}
