use super::entity_player::Player;
use super::resource_player_settings::PlayerSettings;
use super::PlayerState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn jump_player(
    keys: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    player_state: Res<PlayerState>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in player_query.iter_mut() {
        if keys.just_pressed(settings.jump) && player_state.is_in_ground {
            velocity.linvel += Vec3::new(0.0, settings.jump_velocity, 0.0);
        }
    }
}
