use super::entity_player::Player;
use super::resource_player_settings::PlayerSettings;
use super::PlayerState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn jump_player(
    keys: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in player_query.iter_mut() {
        if keys.just_pressed(settings.jump) {
            if player_state.is_in_ground {
                velocity.linvel += Vec3::new(0.0, settings.jump_velocity, 0.0);
            } else if let Some(wall_running) = &player_state.wall_running {
                velocity.linvel += Vec3::new(
                    wall_running.normal_force.x * 5.0,
                    settings.jump_velocity * 0.8,
                    wall_running.normal_force.z * 5.0,
                );
                player_state.inertia = velocity.linvel;
            }
        }
    }
}
