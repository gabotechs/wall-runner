use super::resource_player_settings::PlayerSettings;
use super::resource_player_state::PlayerState;
use crate::player::resource_player_input::PlayerInput;
use crate::player::resource_player_kinematics::PlayerKinematics;
use crate::utils::vec3_horizontal_vec2;
use bevy::prelude::*;
use std::borrow::BorrowMut;
use std::collections::HashSet;

fn get_move_vec(settings: &PlayerSettings, keys: &HashSet<KeyCode>, angle: f32) -> Vec2 {
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

fn jump_out_of_wall(
    move_vector: Vec2,
    normal_force: Vec2,
    wall_run_velocity: Vec2,
    jump_speed: f32,
) -> Vec2 {
    let normal_force = normal_force.clamp_length(jump_speed, wall_run_velocity.length());
    let scaled_normal = normal_force + wall_run_velocity;
    let proj_move_vector = move_vector.project_onto(wall_run_velocity);
    (scaled_normal + proj_move_vector * 0.5) / 2.0
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
        if keys.just_pressed(settings.jump) {
            let jump_velocity = jump_out_of_wall(
                move_vector,
                vec3_horizontal_vec2(wall_running.normal_force),
                wall_run_displacement,
                settings.jump_velocity,
            );
            info!("Jumping out of wall {:?}", jump_velocity);
            kinematics.displacement.add_velocity(jump_velocity);
            kinematics.vertical_impulse += settings.jump_velocity;
            player_state.wall_run_vote = 0;
            player_state.wall_running = None;
        } else {
            let wall_run_speed = if let Some(speed) = wall_running.speed {
                speed
            } else {
                wall_running.speed = Some(prev_frame_velocity.length());
                wall_running.speed.unwrap()
            };
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
