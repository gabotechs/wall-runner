use super::Player;
use crate::player::{PlayerSettings, PlayerState, WallRunningState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::borrow::BorrowMut;

pub fn player_contacts(
    rapier_context: Res<RapierContext>,
    settings: Res<PlayerSettings>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<Entity, With<Player>>,
) {
    let mut is_in_ground = false;
    let mut normal_force = Vec3::default();
    for entity in player_query.iter_mut() {
        for contact_pair in rapier_context.contacts_with(entity) {
            let direction = if contact_pair.collider1() == entity {
                -1.0
            } else {
                1.0
            };
            let best_manifold = contact_pair.find_deepest_contact();
            if let Some((manifold, _)) = best_manifold {
                normal_force = manifold.normal() * direction;
            }
        }
    }
    let has_vertical_forces = normal_force.y > 0.9;
    let has_horizontal_forces = (normal_force.x.powi(2) + normal_force.z.powi(2)).sqrt() > 0.9;
    // if the manifold is positive and vertical, then we are on the ground
    if has_vertical_forces {
        is_in_ground = true;
        // if the sum vector of all the normal forces has mainly an horizontal component, then we are wall running
    } else if has_horizontal_forces && player_state.wall_running.is_none() {
        println!("start wall run");
        println!("{:?}", normal_force);
        let current_direction = Vec2::new(
            player_state.velocity.linvel.x,
            player_state.velocity.linvel.z,
        );
        let speed = current_direction.length();
        let v1 = Vec2::new(-normal_force.z, normal_force.x);
        let v2 = Vec2::new(normal_force.z, -normal_force.x);
        let tangent_direction = if v1.angle_between(current_direction).abs()
            < v2.angle_between(current_direction).abs()
        {
            v1
        } else {
            v2
        };
        let direction = tangent_direction + current_direction;
        let direction = direction.clamp_length_max(1.0) * speed;
        println!("{:?}", direction);
        player_state.wall_running = Some(WallRunningState {
            just_started: true,
            ttl_counter: 0,
            normal_force,
            speed,
            direction: (direction.x, direction.y),
        });
    } else if !has_horizontal_forces && player_state.wall_running.is_some() {
        if let Some(wall_running) = player_state.wall_running.borrow_mut() {
            wall_running.ttl_counter += 1;
            if wall_running.ttl_counter > settings.wall_run_ttl_frames {
                println!("stop wall run");
                player_state.wall_running = None;
            }
        }
    } else if has_horizontal_forces && player_state.wall_running.is_some() {
        if let Some(wall_running) = player_state.wall_running.borrow_mut() {
            wall_running.ttl_counter = 0;
            wall_running.just_started = false;
        }
    }
    player_state.is_in_ground = is_in_ground;
}
