use crate::component_player_state::PlayerState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod compute_forces;
mod compute_in_ground;
mod compute_wall_running;

use crate::PlayerSettings;

pub fn compute_contacts(
    rapier_context: Res<RapierContext>,
    settings: Res<PlayerSettings>,
    mut player_query: Query<(Entity, &mut PlayerState)>,
) {
    for (entity, mut player_state) in player_query.iter_mut() {
        let forces =
            compute_forces::compute_forces(&rapier_context, &settings, entity, &mut player_state);
        compute_in_ground::compute_in_ground(&forces, &settings, &mut player_state);
        compute_wall_running::compute_wall_running(&forces, &settings, &mut player_state)
    }
}
