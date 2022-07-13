use super::resource_player_settings::PlayerSettings;
use super::resource_player_state::PlayerState;
use crate::player::resource_player_input::PlayerInput;
use crate::player::resource_player_kinematics::PlayerKinematics;
use crate::utils::vec3_horizontal_vec2;
use bevy::prelude::*;
use std::borrow::BorrowMut;
use std::collections::HashSet;

pub fn get_move_vec(settings: &PlayerSettings, keys: &HashSet<KeyCode>, angle: f32) -> Vec2 {
    let frontal_speed = settings.speed;
    let lateral_speed = settings.speed * 0.25;
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
    Vec2::new(x, z)
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
    let move_vector = get_move_vec(&settings, &keys_set, player_input.y_angle);
    let prev_frame_velocity = vec3_horizontal_vec2(player_state.velocity.linvel);
    if player_state.is_in_ground {
        // set the velocity
        kinematics.displacement.add_velocity(move_vector);
        if keys.just_pressed(settings.jump) {
            kinematics.vertical_impulse += settings.jump_velocity;
            player_state.is_in_ground = false;
            player_state.ground_vote = 0;
        }
    } else if let Some(wall_running) = player_state.wall_running.borrow_mut() {
        // snap to wall if wall running
        kinematics.gravity = settings.wall_run_gravity;
        let wall_run_displacement = prev_frame_velocity.project_onto(wall_running.direction);
        let wall_run_speed = if let Some(speed) = wall_running.speed {
            speed
        } else {
            wall_running.speed = Some(prev_frame_velocity.length());
            wall_running.speed.unwrap()
        };
        if keys.just_pressed(settings.jump) {
            let normal_force = vec3_horizontal_vec2(wall_running.normal_force);
            let scaled_normal = (normal_force * wall_run_speed).clamp_length_min(settings.speed);
            let proj_move_vector = move_vector.project_onto(wall_running.direction);
            let jump_velocity = 0.5 * scaled_normal + 0.75 * proj_move_vector;
            info!("Jumping out of wall {:?}", jump_velocity);
            kinematics.displacement.add_velocity(jump_velocity);
            kinematics.vertical_impulse += settings.jump_velocity;
            player_state.wall_run_vote = 0;
            player_state.wall_running = None;
        } else {
            kinematics
                .displacement
                .add_velocity(wall_run_displacement.clamp_length_min(wall_run_speed))
        }
    } else {
        // no control of the velocity in the air, the best we can do is force
        kinematics
            .displacement
            .add_force(move_vector.normalize_or_zero() * settings.air_control);
    }
}
