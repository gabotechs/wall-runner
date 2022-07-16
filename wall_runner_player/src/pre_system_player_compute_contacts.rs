use crate::{Player, PlayerSettings, PlayerState, WallRunningState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::borrow::BorrowMut;
use std::f32::consts::PI;
use wall_runner_utils::*;

pub fn player_contacts(
    rapier_context: Res<RapierContext>,
    settings: Res<PlayerSettings>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<Entity, With<Player>>,
) {
    let mut normal_forces: Vec<Vec3> = vec![];
    for entity in player_query.iter_mut() {
        for contact_pair in rapier_context.contacts_with(entity) {
            let direction = if contact_pair.collider1() == entity {
                -1.0
            } else {
                1.0
            };
            let best_manifold = contact_pair.find_deepest_contact();
            if let Some((manifold, _)) = best_manifold {
                normal_forces.push(manifold.normal() * direction);
            }
        }
        player_state.something_above = rapier_context
            .cast_shape(
                player_state.position,
                Rot::default(),
                Vec3::Y,
                &Collider::ball(settings.width),
                (0.5 + 0.25) * settings.height,
                InteractionGroups::all(),
                Some(&|e| e != entity),
            )
            .is_some();
    }
    // if the manifold is positive and vertical, then we are on the ground
    let mut has_vertical_forces = false;
    let mut horizontal_force: Option<Vec3> = None;
    for normal_force in normal_forces {
        if normal_force.y > 0.9 {
            has_vertical_forces = true;
        }
        if (normal_force.x.powi(2) + normal_force.z.powi(2)).sqrt() > 0.9 {
            horizontal_force = Some(normal_force)
        }
    }
    if has_vertical_forces && !player_state.is_in_ground {
        if player_state.ground_vote < settings.ground_votes {
            player_state.ground_vote += settings.ground_up_vote;
            debug!("[+] vote ground {}", player_state.ground_vote);
        }
        if player_state.ground_vote >= settings.ground_votes {
            info!("in ground");
            player_state.is_in_ground = true;
        }
    } else if has_vertical_forces && player_state.is_in_ground {
        player_state.ground_vote = settings.ground_votes;
    } else if !has_vertical_forces && player_state.is_in_ground {
        if player_state.ground_vote > 0 {
            player_state.ground_vote -= settings.ground_down_vote;
            debug!("[-] vote ground {}", player_state.ground_vote);
        }
        if player_state.ground_vote == 0 {
            info!("not in ground");
            player_state.is_in_ground = false;
        }
    } else if !has_vertical_forces && !player_state.is_in_ground {
        player_state.ground_vote = 0;
    }

    // if the sum vector of all the normal forces has mainly an horizontal component, then we are wall running
    let can_be_wall_running = horizontal_force.is_some() && !player_state.is_in_ground;
    if can_be_wall_running && player_state.wall_running.is_none() {
        if player_state.wall_run_vote < settings.wall_run_votes {
            player_state.wall_run_vote += settings.wall_run_up_vote;
            debug!("[+] vote wall run {}", player_state.wall_run_vote);
        }
        if player_state.wall_run_vote >= settings.wall_run_votes {
            const ANGLE_EPSILON: f32 = 0.05;
            let tangent_direction = nearest_with_angle(
                vec3_horizontal_vec2(horizontal_force.unwrap()),
                vec3_horizontal_vec2(player_state.velocity.linvel),
                PI / 2.0 + ANGLE_EPSILON,
            );
            info!("start wall run,  direction: {:?}", tangent_direction);
            player_state.wall_running = Some(WallRunningState {
                speed: None,
                just_started: true,
                normal_force: horizontal_force.unwrap(),
                direction: tangent_direction.normalize_or_zero(),
            });
        }
    } else if !can_be_wall_running && player_state.wall_running.is_some() {
        if player_state.wall_run_vote > 0 {
            player_state.wall_run_vote -= settings.wall_run_down_vote;
            debug!("[-] vote wall run {}", player_state.wall_run_vote);
        }
        if player_state.wall_run_vote == 0 {
            info!("stop wall run");
            player_state.wall_running = None;
        }
    } else if let Some(wall_running) = player_state.wall_running.borrow_mut() {
        wall_running.just_started = false;
    }
}
