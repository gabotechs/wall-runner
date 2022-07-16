use super::entity_camera::GameCamera;
use super::resource_camera_settings::*;
use super::resource_camera_state::*;
use crate::camera::CameraInput;
use crate::input::InputEvent;
use crate::utils::read_one_event;
use bevy::prelude::*;

/// Handles looking around if cursor is locked
pub fn camera_look(
    settings: Res<CameraSettings>,
    windows: Res<Windows>,
    time: Res<Time>,
    camera_input: Res<CameraInput>,
    mut camera_state: ResMut<CameraState>,
    input_ev_reader: EventReader<InputEvent>,
    mut query: Query<&mut Transform, With<GameCamera>>,
) {
    let input_ev = read_one_event(input_ev_reader);
    if camera_input.inactive {
        return;
    }
    let window = windows.get_primary().unwrap();
    let scale = window.height().min(window.width());
    let scale = time.delta().as_secs_f32();
    for mut transform in query.iter_mut() {
        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
        camera_state.pitch -= (settings.sensitivity * input_ev.look.y * scale).to_radians();
        camera_state.yaw -= (settings.sensitivity * input_ev.look.x * scale).to_radians();
        camera_state.pitch = camera_state.pitch.clamp(-1.54, 1.54);

        // Order is important to prevent unintended roll
        let yaw_quat = Quat::from_axis_angle(Vec3::Y, camera_state.yaw);
        let pitch_quat = Quat::from_axis_angle(Vec3::X, camera_state.pitch);
        let tilt_quat = Quat::from_axis_angle(Vec3::Z, camera_state.tilt);
        transform.rotation = yaw_quat * pitch_quat * tilt_quat;
    }
}
