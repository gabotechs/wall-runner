use crate::{CameraInput, CameraState, GameCamera};
use bevy::prelude::*;

const STEP: f32 = 8.0;

pub fn camera_position(
    camera_input: Res<CameraInput>,
    time: Res<Time>,
    mut camera_state: ResMut<CameraState>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
) {
    let step = STEP * time.delta().as_secs_f32();
    for mut transform in camera_query.iter_mut() {
        transform.translation = camera_input.position;

        let diff_x = camera_input.position_offset.x - camera_state.offset.x;
        let diff_x = diff_x.clamp(-step, step);
        camera_state.offset.x += diff_x;
        transform.translation.x = camera_input.position.x + camera_state.offset.x;

        let diff_y = camera_input.position_offset.y - camera_state.offset.y;
        let diff_y = diff_y.clamp(-step, step);
        camera_state.offset.y += diff_y;
        transform.translation.y = camera_input.position.y + camera_state.offset.y;

        let diff_z = camera_input.position_offset.z - camera_state.offset.z;
        let diff_z = diff_z.clamp(-step, step);
        camera_state.offset.z += diff_z;
        transform.translation.z = camera_input.position.z + camera_state.offset.z;
    }
}
