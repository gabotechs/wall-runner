use super::entity_camera::GameCamera;
use super::resource_camera_settings::*;
use super::resource_camera_state::*;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

/// Handles looking around if cursor is locked
pub fn camera_look(
    settings: Res<CameraSettings>,
    windows: Res<Windows>,
    mut camera_state: ResMut<CameraState>,
    mut reader_motion: ResMut<ManualEventReader<MouseMotion>>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<GameCamera>>,
) {
    let window = windows.get_primary().unwrap();
    let scale = window.height().min(window.width());
    for mut transform in query.iter_mut() {
        for ev in reader_motion.iter(&motion) {
            // Using smallest of height or width ensures equal vertical and horizontal sensitivity
            camera_state.pitch -= (settings.sensitivity * ev.delta.y * scale).to_radians();
            camera_state.yaw -= (settings.sensitivity * ev.delta.x * scale).to_radians();
            camera_state.pitch = camera_state.pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            let yaw_quat = Quat::from_axis_angle(Vec3::Y, camera_state.yaw);
            let pitch_quat = Quat::from_axis_angle(Vec3::X, camera_state.pitch);
            let tilt_quat = Quat::from_axis_angle(Vec3::Z, camera_state.tilt);
            transform.rotation = yaw_quat * pitch_quat * tilt_quat;
        }
    }
}
