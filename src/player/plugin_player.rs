use super::pre_system_player_compute_contacts::player_contacts;
use super::resource_player_settings::PlayerSettings;
use super::resource_player_state::PlayerState;
use super::system_player_move::move_player;
use crate::player::post_system_player_dump_kinematics::player_dump_kinematics;
use crate::player::post_system_player_sync_state_position::player_sync_state_position;
use crate::player::pre_system_player_apply_input::player_apply_input;
use crate::player::pre_system_player_init_kinematics::{player_reset_kinematics, PlayerKinematics};
use crate::player::pre_system_player_load_physics_into_state::player_load_physics_into_state;
use crate::player::resource_player_input::PlayerInput;
use crate::player::startup_system_player_setup::setup_player;
use crate::player::system_player_audio::{player_audio, LandAudio, PlayerAudioState, WallRunAudio};
use crate::player::system_player_crouch::player_crouch;
use crate::player::system_player_reset::reset_player;
use bevy::prelude::*;
use bevy_kira_audio::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .init_resource::<PlayerInput>()
            .init_resource::<PlayerSettings>()
            .init_resource::<PlayerAudioState>()
            .init_resource::<PlayerKinematics>()
            .add_audio_channel::<WallRunAudio>()
            .add_audio_channel::<LandAudio>()
            .add_startup_system(setup_player)
            .add_system_to_stage(CoreStage::PreUpdate, reset_player)
            .add_system_to_stage(CoreStage::PreUpdate, player_load_physics_into_state)
            .add_system_to_stage(CoreStage::PreUpdate, player_contacts)
            .add_system_to_stage(CoreStage::PreUpdate, player_reset_kinematics)
            .add_system_to_stage(CoreStage::PreUpdate, player_apply_input)
            .add_system(move_player)
            .add_system(player_crouch)
            .add_system(player_audio)
            .add_system_to_stage(CoreStage::PostUpdate, player_sync_state_position)
            .add_system_to_stage(CoreStage::PostUpdate, player_dump_kinematics);
    }
}
