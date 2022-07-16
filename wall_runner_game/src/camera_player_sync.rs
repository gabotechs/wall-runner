use bevy::prelude::*;

use wall_runner_camera::*;
use wall_runner_player::*;
use wall_runner_utils::*;

pub fn attach_camera_to_player(
    camera_state: ResMut<CameraState>,
    player_state: Res<PlayerState>,
    mut camera_input: ResMut<CameraInput>,
    mut player_input: ResMut<PlayerInput>,
    player_settings: Res<PlayerSettings>,
) {
    const TILT_ANGLE_FACTOR: f32 = 0.4;
    const HEAD_HORIZONTAL_OFFSET: f32 = 0.2;
    camera_input.position = player_state.position;
    camera_input.position_offset.y = player_state.head_offset;
    camera_input.position_offset.x = camera_state.yaw.sin() * HEAD_HORIZONTAL_OFFSET;
    camera_input.position_offset.z = camera_state.yaw.cos() * HEAD_HORIZONTAL_OFFSET;
    if let Some(wall_running) = &player_state.wall_running {
        let wall_vector = Vec2::new(wall_running.normal_force.x, wall_running.normal_force.z);
        let move_vector = vec3_horizontal_vec2(player_state.velocity.linvel);
        let angle = wall_vector.angle_between(move_vector);
        if !angle.is_nan() {
            camera_input.tilt_angle =
                angle.sin() * move_vector.length() / player_settings.speed * TILT_ANGLE_FACTOR;
        }
    } else {
        camera_input.tilt_angle = 0.0;
    }
    player_input.y_angle = camera_state.yaw;
}
