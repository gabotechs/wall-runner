use crate::component_player_kinematics::PlayerKinematics;
use crate::component_player_state::PlayerState;
use crate::{Player, PlayerControlEvent, PlayerInput, PlayerSettings};
use bevy::prelude::*;
use std::borrow::BorrowMut;
use wall_runner_utils::{read_one_event, rotate_vec, vec3_horizontal_vec2};

fn jump_out_of_wall(
    move_vector: Vec2,
    normal_force: Vec2,
    wall_run_velocity: Vec2,
    jump_speed: f32,
) -> Vec2 {
    let normal_force = normal_force.clamp_length(jump_speed, wall_run_velocity.length());
    let scaled_normal = normal_force + wall_run_velocity;
    let proj_move_vector = move_vector.project_onto(wall_run_velocity);
    (scaled_normal + proj_move_vector * 0.5) / 1.5
}

pub fn move_player(
    input_ev_reader: EventReader<PlayerControlEvent>,
    settings: Res<PlayerSettings>,
    mut player_query: Query<(&PlayerInput, &mut PlayerState, &mut PlayerKinematics), With<Player>>,
) {
    let input_ev = read_one_event(input_ev_reader);
    for (player_input, mut player_state, mut kinematics) in player_query.iter_mut() {
        let move_vector = Vec2::new(
            input_ev.movement.x * 0.25,
            input_ev.movement.y.clamp(-0.5, 1.0),
        );
        let move_vector = -1.0 * move_vector; // don't know why, but this is inverted
        let move_vector = rotate_vec(move_vector, -player_input.y_angle);
        let move_vector = move_vector * settings.run_speed;
        let prev_frame_velocity = vec3_horizontal_vec2(player_state.velocity.linvel);
        if player_state.is_in_ground {
            // set the velocity
            kinematics.displacement.add_velocity(move_vector);
            if input_ev.jump {
                kinematics.vertical_impulse += settings.jump_speed;
                player_state.is_in_ground = false;
                player_state.ground_vote = -settings.ground_up_vote;
            }
        } else if let Some(wall_running) = player_state.wall_running.borrow_mut() {
            // snap to wall if wall running
            kinematics.gravity = settings.wall_run_gravity;
            let wall_run_displacement = prev_frame_velocity.project_onto(wall_running.direction);
            if input_ev.jump {
                let jump_velocity = jump_out_of_wall(
                    move_vector,
                    vec3_horizontal_vec2(wall_running.normal_force),
                    wall_run_displacement,
                    settings.jump_speed,
                );
                info!("Jumping out of wall {:?}", jump_velocity);
                kinematics.displacement.add_velocity(jump_velocity);
                kinematics.vertical_impulse += settings.jump_speed;
                player_state.wall_run_vote = -settings.wall_run_up_vote;
                player_state.wall_running = None;
            } else {
                let wall_run_speed = if let Some(speed) = wall_running.speed {
                    speed
                } else {
                    info!(
                        "locking wall run speed to {}",
                        wall_run_displacement.length()
                    );
                    wall_running.speed = Some(wall_run_displacement.length());
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
}
