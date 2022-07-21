use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod compute_forces;
mod compute_in_ground;
mod compute_wall_running;

use crate::{Player, PlayerSettings, PlayerState};

pub fn compute_contacts(
    rapier_context: Res<RapierContext>,
    settings: Res<PlayerSettings>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<Entity, With<Player>>,
) {
    let forces = compute_forces::compute_forces(
        &rapier_context,
        &settings,
        &mut player_state,
        &mut player_query,
    );
    compute_in_ground::compute_in_ground(&forces, &settings, &mut player_state);
    compute_wall_running::compute_wall_running(&forces, &settings, &mut player_state)
}
