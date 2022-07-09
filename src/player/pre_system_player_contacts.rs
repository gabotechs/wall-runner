use super::Player;
use crate::player::{PlayerState, WallRunningState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn player_contacts(
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<Entity, With<Player>>,
) {
    let mut is_in_ground = false;
    let mut normal_sum = Vec3::default();
    for entity in player_query.iter_mut() {
        for contact_pair in rapier_context.contacts_with(entity) {
            let direction = if contact_pair.collider1() == entity {
                -1.0
            } else {
                1.0
            };
            for manifold in contact_pair.manifolds() {
                let normal = manifold.normal() * direction;
                // if at least one of the manifolds is positive and vertical, then we are on the ground
                if normal.y > 0.9 {
                    is_in_ground = true;
                }
                normal_sum += normal;
            }
        }
    }
    normal_sum = normal_sum.normalize();

    // if the sum vector of all the normal forces has mainly an horizontal component, then we are wall running
    if (normal_sum.x.powi(2) + normal_sum.z.powi(2)).sqrt() > 0.95 {
        if let Some(wall_running) = &player_state.wall_running {
            player_state.wall_running = Some(WallRunningState {
                start_ms: wall_running.start_ms,
                normal_force: normal_sum,
            })
        } else {
            println!("start wall run");
            player_state.wall_running = Some(WallRunningState {
                start_ms: time.time_since_startup().as_millis(),
                normal_force: normal_sum,
            })
        }
    } else {
        if player_state.wall_running.is_some() {
            println!("stop wall run");
        }
        player_state.wall_running = None;
    }
    player_state.is_in_ground = is_in_ground;
}
