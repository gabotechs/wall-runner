use crate::post_system_player_dump_kinematics::player_dump_kinematics;
use crate::pre_system_player_compute_contacts::player_contacts;
use crate::resource_player_kinematics::PlayerKinematics;
use crate::startup_system_player_setup::setup_player;
use crate::system_player_audio::{player_audio, LandAudio, PlayerAudioState, WallRunAudio};
use crate::system_player_crouch::player_crouch;
use crate::system_player_move::move_player;
use crate::system_player_reset::{reset_player, ResetAudio};
use crate::*;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy_kira_audio::*;

pub struct PlayerPlugin;

fn pause(input: Res<PlayerInput>) -> ShouldRun {
    if input.inactive {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .init_resource::<PlayerInput>()
            .init_resource::<PlayerSettings>()
            .init_resource::<PlayerAudioState>()
            .init_resource::<PlayerKinematics>()
            .add_event::<PlayerControlEvent>()
            .add_audio_channel::<WallRunAudio>()
            .add_audio_channel::<LandAudio>()
            .add_audio_channel::<ResetAudio>()
            .add_startup_system(setup_player)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_run_criteria(pause)
                    .with_system(reset_player)
                    .with_system(player_contacts),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(pause)
                    .with_system(move_player)
                    .with_system(player_crouch)
                    .with_system(player_audio),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(pause)
                    .with_system(player_dump_kinematics),
            );
    }
}
