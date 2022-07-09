use super::pre_system_player_contacts::player_contacts;
use super::resource_player_settings::PlayerSettings;
use super::resource_player_state::PlayerState;
use super::system_player_jump::jump_player;
use super::system_player_move::move_player;
use crate::player::post_system_player_update_state::player_update_state;
use crate::player::resource_player_input::PlayerInput;
use crate::player::startup_system_player_setup::setup_player;
use crate::player::system_player_gravity::update_gravity;
use crate::player::system_player_reset::reset_player;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .init_resource::<PlayerInput>()
            .init_resource::<PlayerSettings>()
            .add_startup_system(setup_player)
            .add_system_to_stage(CoreStage::PreUpdate, player_contacts)
            .add_system(update_gravity)
            .add_system(jump_player)
            .add_system(move_player)
            .add_system(reset_player)
            .add_system_to_stage(CoreStage::PostUpdate, player_update_state);
    }
}
