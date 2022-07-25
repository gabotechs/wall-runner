use crate::component_player_kinematics::PlayerKinematics;
use crate::{Player, PlayerControlEvent, PlayerSettings, PlayerState};
use bevy::prelude::*;
use wall_runner_utils::{read_one_event, vec3_horizontal_vec2};

const EPSILON: f32 = 0.02;

pub fn player_crouch(
    settings: Res<PlayerSettings>,
    time: Res<Time>,
    input_ev_reader: EventReader<PlayerControlEvent>,
    mut player_query: Query<
        (&mut Transform, &mut PlayerState, &mut PlayerKinematics),
        With<Player>,
    >,
) {
    let input_ev = read_one_event(input_ev_reader);
    for (mut transform, mut player_state, mut kinematics) in player_query.iter_mut() {
        if input_ev.is_crouching && player_state.is_in_ground {
            player_state.crouch_state.is_crouching = true;
            let speed_boost = settings.crouch_boost * player_state.crouch_state.charge;
            let current_dir =
                vec3_horizontal_vec2(player_state.velocity.linvel).normalize_or_zero();
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
                transform.scale.y = settings.height_crouching / settings.height;
                transform.translation.y -=
                    (settings.height - settings.height_crouching) * 0.5 - EPSILON;
            }
            player_state.head_offset = -0.1;
        } else {
            if player_state.something_above {
                return;
            }
            player_state.crouch_state.is_crouching = false;
            if player_state.crouch_state.charge >= 1.0 {
                player_state.crouch_state.charge = 1.0;
            } else {
                player_state.crouch_state.charge +=
                    settings.crouch_recharge_multiplier * time.delta().as_secs_f32();
            }
            if player_state.head_offset < 0.0 {
                transform.scale.y = 1.0;
                transform.translation.y +=
                    (settings.height - settings.height_crouching) * 0.5 + EPSILON;
            }
            player_state.head_offset = 1.0;
        }
    }
}
