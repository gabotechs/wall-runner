use super::entity_player::Player;
use super::resource_player_settings::PlayerSettings;
use super::system_player_move::get_move_vec;
use super::PlayerState;
use crate::player::PlayerInput;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::collections::HashSet;

pub fn jump_player(
    keys: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    mut player_state: ResMut<PlayerState>,
    player_input: Res<PlayerInput>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in player_query.iter_mut() {
        if keys.just_pressed(settings.jump) {
            if player_state.is_in_ground {
                velocity.linvel += Vec3::new(0.0, settings.jump_velocity, 0.0);
            } else if let Some(wall_running) = &player_state.wall_running {
                let mut keys_set: HashSet<KeyCode> = HashSet::new();
                for key in keys.get_pressed() {
                    keys_set.insert(*key);
                }
                let move_vec = get_move_vec(&settings, &keys_set, player_input.y_angle);
                // todo: empirical values
                velocity.linvel = Vec3::new(
                    wall_running.normal_force.x + move_vec.0 / 3.0,
                    velocity.linvel.y + settings.jump_velocity * 0.8,
                    wall_running.normal_force.z + move_vec.1 / 3.0,
                );
                player_state.wall_running = None;
            }
        }
    }
}
