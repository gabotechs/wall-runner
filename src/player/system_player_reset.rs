use crate::player::{Player, PlayerInput, PlayerSettings, PlayerState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn reset_player(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut ExternalForce), With<Player>>,
    mut player_input: ResMut<PlayerInput>,
    mut player_state: ResMut<PlayerState>,
    settings: Res<PlayerSettings>,
) {
    if player_input.reset {
        info!("resetting player");
        for (mut transform, mut velocity, mut force) in player_query.iter_mut() {
            transform.translation = settings.initial_position;
            velocity.linvel = Vec3::default();
            force.force = Vec3::default();
        }
        player_state.reset();
        player_input.reset = false;
    }
}
