use crate::component_player_state::PlayerState;
use crate::PlayerSettings;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Default)]
pub struct PlayerForces {
    pub has_vertical_forces: bool,
    pub horizontal_force: Option<Vec3>,
}

pub(crate) fn compute_forces(
    rapier_context: &RapierContext,
    settings: &PlayerSettings,
    entity: Entity,
    state: &mut PlayerState,
) -> PlayerForces {
    let mut normal_forces: Vec<Vec3> = vec![];
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
    state.something_above = rapier_context
        .cast_shape(
            state.position,
            Rot::default(),
            Vec3::Y,
            &Collider::ball(settings.width),
            (0.5 + 0.25) * settings.height,
            InteractionGroups::all(),
            Some(&|e| e != entity),
        )
        .is_some();
    // if the manifold is positive and vertical, then we are on the ground
    let mut player_forces = PlayerForces::default();
    for normal_force in normal_forces {
        if normal_force.y > 0.9 {
            player_forces.has_vertical_forces = true;
        }
        if (normal_force.x.powi(2) + normal_force.z.powi(2)).sqrt() > 0.9 {
            player_forces.horizontal_force = Some(normal_force)
        }
    }
    player_forces
}
