use super::Player;
use crate::player::PlayerState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn player_contacts(
    rapier_context: Res<RapierContext>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<(&Velocity, Entity), With<Player>>,
) {
    let mut is_in_ground = false;
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
                }
            }
        }
        if is_in_ground {
            player_state.inertia = velocity.linvel;
        }
        player_state.is_in_ground = is_in_ground;
    }
}
