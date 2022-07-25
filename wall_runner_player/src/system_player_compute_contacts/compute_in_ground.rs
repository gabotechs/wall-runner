use crate::component_player_state::PlayerState;
use crate::system_player_compute_contacts::compute_forces::PlayerForces;
use crate::PlayerSettings;
use bevy::prelude::*;

pub fn compute_in_ground(
    player_forces: &PlayerForces,
    settings: &PlayerSettings,
    player_state: &mut PlayerState,
) {
    if player_forces.has_vertical_forces && !player_state.is_in_ground {
        if player_state.ground_vote < settings.ground_votes {
            player_state.ground_vote += settings.ground_up_vote;
            debug!("[+] vote ground {}", player_state.ground_vote);
        }
        if player_state.ground_vote >= settings.ground_votes {
            info!("in ground");
            player_state.is_in_ground = true;
        }
    } else if player_forces.has_vertical_forces && player_state.is_in_ground {
        player_state.ground_vote = settings.ground_votes;
    } else if !player_forces.has_vertical_forces && player_state.is_in_ground {
        if player_state.ground_vote > 0 {
            player_state.ground_vote -= settings.ground_down_vote;
            debug!("[-] vote ground {}", player_state.ground_vote);
        }
        if player_state.ground_vote == 0 {
            info!("not in ground");
            player_state.is_in_ground = false;
        }
    } else if !player_forces.has_vertical_forces && !player_state.is_in_ground {
        player_state.ground_vote = 0;
    }
}
