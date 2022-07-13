use super::resource_player_settings::PlayerSettings;
use super::resource_player_state::PlayerState;
use crate::player::resource_player_input::PlayerInput;
use crate::player::resource_player_kinematics::PlayerKinematics;
use crate::utils::vec3_horizontal_vec2;
use bevy::prelude::*;
use std::borrow::BorrowMut;
use std::collections::HashSet;

pub fn get_move_vec(settings: &PlayerSettings, keys: &HashSet<KeyCode>, angle: f32) -> (f32, f32) {
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
    let (x, z) = get_move_vec(&settings, &keys_set, player_input.y_angle);
    let prev_frame_displacement = vec3_horizontal_vec2(player_state.velocity.linvel);
    if player_state.is_in_ground {
        // set the velocity
        kinematics.displacement.add_velocity(Vec2::new(x, z));
        if keys.just_pressed(settings.jump) {
            kinematics.vertical_impulse += settings.jump_velocity;
            player_state.is_in_ground = false;
            player_state.ground_vote = 0;
        }
    } else if let Some(wall_running) = player_state.wall_running.borrow_mut() {
        // snap to wall if wall running
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
                jump_direction.x + 0.7 * x / settings.speed,
                jump_direction.y + 0.7 * z / settings.speed,
            )
            .clamp_length_min(0.75 * wall_run_speed);
            info!("Jumping out of wall {:?}", jump_displacement);
            kinematics.displacement.add_velocity(jump_displacement);
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
        let v = Vec2::new(x, z).normalize_or_zero();
        kinematics.displacement.add_force(v * settings.air_control);
    }
}
