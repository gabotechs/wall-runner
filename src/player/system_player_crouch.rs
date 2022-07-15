use crate::player::resource_player_kinematics::PlayerKinematics;
use crate::player::{Player, PlayerSettings, PlayerState};
use crate::utils::vec3_horizontal_vec2;
use bevy::prelude::*;

pub fn player_crouch(
    keys: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    time: Res<Time>,
    mut player_state: ResMut<PlayerState>,
    mut kinematics: ResMut<PlayerKinematics>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if keys.pressed(settings.crouch) && player_state.is_in_ground {
        let speed_boost = settings.crouch_boost * player_state.crouch_state.charge;
        let current_dir = vec3_horizontal_vec2(player_state.velocity.linvel).normalize_or_zero();
        kinematics
            .displacement
            .add_velocity(current_dir * speed_boost);
        if player_state.crouch_state.charge <= 0.0 {
            player_state.crouch_state.charge = 0.0;
        } else {
            player_state.crouch_state.charge -=
                settings.crouch_discharge_multiplier * time.delta().as_secs_f32();
        }
        if player_state.head_offset > 0.0 {
            for mut player in player_query.iter_mut() {
                player.scale.y = 0.5;
                player.translation.y -= settings.height * 0.25;
            }
        }
        player_state.head_offset = -0.1;
    } else {
        if player_state.something_above {
            return;
        }
        if player_state.crouch_state.charge >= 1.0 {
            player_state.crouch_state.charge = 1.0;
        } else {
            player_state.crouch_state.charge +=
                settings.crouch_recharge_multiplier * time.delta().as_secs_f32();
        }
        const STAND_EXTRA_EPSILON: f32 = 0.01;
        if player_state.head_offset < 0.0 {
            for mut player in player_query.iter_mut() {
                player.scale.y = 1.0;
                player.translation.y += settings.height * 0.25 + STAND_EXTRA_EPSILON;
            }
        }
        player_state.head_offset = 1.0;
    }
}
