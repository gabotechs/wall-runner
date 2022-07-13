use super::resource_player_settings::PlayerSettings;
use super::resource_player_state::PlayerState;
use crate::player::pre_system_player_init_kinematics::PlayerKinematics;
use crate::player::resource_player_input::PlayerInput;
use crate::utils::vec3_horizontal_vec2;
use bevy::prelude::*;
use std::borrow::BorrowMut;
use std::collections::HashSet;

pub fn get_move_vec(settings: &PlayerSettings, keys: &HashSet<KeyCode>, angle: f32) -> (f32, f32) {
    let frontal_speed = settings.speed;
    let lateral_speed = settings.speed * 0.5;
    let mut x = 0f32;
    let mut z = 0f32;
    if keys.contains(&settings.forward) {
        x -= frontal_speed * angle.sin();
        z -= frontal_speed * angle.cos();
    }
    if keys.contains(&settings.backward) {
        x += frontal_speed * angle.sin();
        z += frontal_speed * angle.cos();
    }
    if keys.contains(&settings.left) {
        x -= lateral_speed * angle.cos();
        z += lateral_speed * angle.sin();
    }
    if keys.contains(&settings.right) {
        x += lateral_speed * angle.cos();
        z -= lateral_speed * angle.sin();
    }
    (x, z)
}

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    player_input: Res<PlayerInput>,
    mut player_state: ResMut<PlayerState>,
    mut kinematics: ResMut<PlayerKinematics>,
) {
    let mut keys_set: HashSet<KeyCode> = HashSet::new();
    for key in keys.get_pressed() {
        keys_set.insert(*key);
    }
    let f = settings.acceleration_factor;
    let (x, z) = get_move_vec(&settings, &keys_set, player_input.y_angle);
    let prev_frame_displacement = player_state.kinematics.displacement;
    if player_state.is_in_ground {
        // set the velocity
        kinematics.displacement.x += f * x + (1.0 - f) * player_state.kinematics.displacement.x;
        kinematics.displacement.y += f * z + (1.0 - f) * player_state.kinematics.displacement.y;
        if keys.just_pressed(settings.jump) {
            kinematics.vertical_impulse += settings.jump_velocity;
        }
    } else if let Some(wall_running) = player_state.wall_running.borrow_mut() {
        // snap to wall if wall running, this is done by locking the velocity to the wall run direction.
        // The current running velocity must be projected onto the wall run direction, but this last
        // vector is slightly rotated towards the wall, so each frame the velocity is projected, even
        // if we clamp_length_min it, rapier is going to truncate it because we cannot insert ourselves
        // into the wall. By registering the first speed, we force the speed to be the first one
        // registered during the wall run
        kinematics.gravity = settings.wall_run_gravity;
        let wall_run_displacement = prev_frame_displacement.project_onto(wall_running.direction);
        let wall_run_speed = if let Some(speed) = wall_running.speed {
            speed
        } else {
            wall_running.speed = Some(wall_run_displacement.length());
            wall_running.speed.unwrap()
        };
        if keys.just_pressed(settings.jump) {
            let normal_force = vec3_horizontal_vec2(wall_running.normal_force);
            let wall_run_vector = wall_run_displacement.normalize_or_zero();
            let jump_direction = (normal_force + wall_run_vector).normalize_or_zero();
            let jump_displacement = Vec2::new(
                jump_direction.x + 0.9 * x / settings.speed,
                jump_direction.y + 0.9 * z / settings.speed,
            )
            .clamp_length(wall_run_speed, wall_run_speed);
            info!("Jumping out of wall {:?}", jump_displacement);
            kinematics.displacement += jump_displacement;
            kinematics.vertical_impulse += settings.jump_velocity;
            player_state.wall_run_vote = 0;
            player_state.wall_running = None;
        } else {
            kinematics.displacement += wall_run_displacement.clamp_length_min(wall_run_speed);
        }
    } else {
        // no control of the velocity in the air, the best we can do is force
        let v = Vec2::new(x, z).normalize_or_zero();
        if v.x > 0.0 && v.y > 0.0 {
            kinematics.force.x += settings.air_control * v.x;
            kinematics.force.z += settings.air_control * v.y;
        } else {
            kinematics.displacement = prev_frame_displacement;
        }
    }
}
