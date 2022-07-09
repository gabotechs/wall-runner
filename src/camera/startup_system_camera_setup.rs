use super::entity_camera::*;
use super::resource_camera_state::*;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands, initial_state: Res<CameraState>) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(
                initial_state.position.x,
                initial_state.position.y,
                initial_state.position.z,
            )
            .with_rotation(
                Quat::from_axis_angle(Vec3::Y, initial_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, initial_state.pitch),
            ),
            ..Default::default()
        })
        .insert(Camera);
}
