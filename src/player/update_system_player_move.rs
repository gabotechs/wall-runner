use super::entity_player::Player;
use super::resource_player_settings::PlayerSettings;
use super::resource_player_state::PlayerState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::collections::HashSet;

fn get_move_vec(settings: &PlayerSettings, keys: &HashSet<KeyCode>, angle: f32) -> (f32, f32) {
    let forward_speed = if keys.contains(&settings.run) {
        settings.run_speed
    } else {
        settings.walk_speed
    };
    let backward_speed = settings.walk_speed;
    let lateral_speed = settings.walk_speed * 0.5;
    let mut x = 0f32;
    let mut z = 0f32;
    if keys.contains(&settings.forward) {
        x -= forward_speed * angle.sin();
        z -= forward_speed * angle.cos();
    }
    if keys.contains(&settings.backward) {
        x += backward_speed * angle.sin();
        z += backward_speed * angle.cos();
    }
    if keys.contains(&settings.left) {
        x -= lateral_speed * angle.cos();
        z += lateral_speed * angle.sin();
    }
    if keys.contains(&settings.right) {
        x += lateral_speed * angle.cos();
        z -= lateral_speed * angle.sin();
    }
    (x, z)
}

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    player_state: Res<PlayerState>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    let mut keys_set: HashSet<KeyCode> = HashSet::new();
    for key in keys.get_pressed() {
        keys_set.insert(*key);
    }
    for mut velocity in player_query.iter_mut() {
        let (x, z) = get_move_vec(&settings, &keys_set, player_state.y_angle);
        if player_state.is_in_ground {
            let f = settings.acceleration_factor;
            velocity.linvel.x = f * x + (1.0 - f) * velocity.linvel.x;
            velocity.linvel.z = f * z + (1.0 - f) * velocity.linvel.z;
        } else {
            velocity.linvel.x = player_state.inertia.x + settings.air_control * x;
            velocity.linvel.z = player_state.inertia.z + settings.air_control * z;
        }
    }
}
