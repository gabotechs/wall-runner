use crate::player::resource_player_kinematics::PlayerKinematics;
use crate::player::{PlayerSettings, PlayerState};
use bevy::prelude::*;

pub fn player_crouch(
    keys: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    time: Res<Time>,
    mut player_state: ResMut<PlayerState>,
    mut kinematics: ResMut<PlayerKinematics>,
) {
    if keys.pressed(settings.crouch) && player_state.is_in_ground {
        let speed_boost = settings.crouch_boost * player_state.crouch_state.charge;
        let current_dir = player_state.kinematics.displacement.normalize_or_zero();
        kinematics.displacement.x += current_dir.x * speed_boost;
        kinematics.displacement.y += current_dir.y * speed_boost;
        if player_state.crouch_state.charge <= 0.0 {
            player_state.crouch_state.charge = 0.0;
        } else {
            player_state.crouch_state.charge -=
                settings.crouch_discharge_multiplier * time.delta().as_secs_f32();
        }
        player_state.head_offset = -0.5;
    } else {
        if player_state.crouch_state.charge >= 1.0 {
            player_state.crouch_state.charge = 1.0;
        } else {
            player_state.crouch_state.charge +=
                settings.crouch_recharge_multiplier * time.delta().as_secs_f32();
        }
        player_state.head_offset = 1.0;
    }
}
