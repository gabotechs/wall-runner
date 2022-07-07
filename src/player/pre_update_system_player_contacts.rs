use super::Player;
use crate::player::{PlayerState, WallRunningState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn player_contacts(
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<(&Velocity, Entity), With<Player>>,
) {
    let mut is_in_ground = false;
    let mut wall_running_normal: Option<Vec3> = None;
    for (&velocity, entity) in player_query.iter_mut() {
        for contact_pair in rapier_context.contacts_with(entity) {
            let direction = if contact_pair.collider1() == entity {
                -1.0
            } else {
                1.0
            };
            for manifold in contact_pair.manifolds() {
                let normal = manifold.normal() * direction;
                if normal.y > 0.95 {
                    is_in_ground = true;
                } else if (normal.x.powi(2) + normal.z.powi(2)).sqrt() > 0.95 {
                    wall_running_normal = Some(normal);
                }
            }
        }
        if is_in_ground {
            player_state.inertia = velocity.linvel;
        }
        if let Some(normal_force) = wall_running_normal {
            if let Some(wall_running) = &player_state.wall_running {
                player_state.wall_running = Some(WallRunningState {
                    start_ms: wall_running.start_ms,
                    normal_force,
                })
            } else {
                player_state.wall_running = Some(WallRunningState {
                    start_ms: time.time_since_startup().as_millis(),
                    normal_force,
                })
            }
        } else {
            player_state.wall_running = None;
        }
        player_state.is_in_ground = is_in_ground;
    }
}
