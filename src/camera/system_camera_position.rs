use bevy::prelude::*;
use crate::camera::{CameraInput, GameCamera};

pub fn camera_position(
    camera_input: Res<CameraInput>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>
) {
    for mut transform in camera_query.iter_mut() {
        transform.translation = camera_input.position;
    }
}