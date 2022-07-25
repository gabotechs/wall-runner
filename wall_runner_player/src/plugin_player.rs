use crate::system_player_audio::{player_audio, LandAudio, PlayerAudioState, WallRunAudio};
use crate::system_player_compute_contacts::compute_contacts;
use crate::system_player_crouch::player_crouch;
use crate::system_player_dump_kinematics::player_dump_kinematics;
use crate::system_player_move::move_player;
use crate::system_player_reset::{reset_player, ResetAudio};
use crate::system_player_setup::setup_player;
use crate::*;
use bevy::prelude::*;
use bevy_kira_audio::*;
use wall_runner_events::*;

pub struct PlayerPlugin;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
enum Labels {
    Contacts,
    Kinematics,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSettings>()
            .init_resource::<PlayerAudioState>()
            .add_event::<PlayerControlEvent>()
            .add_audio_channel::<WallRunAudio>()
            .add_audio_channel::<LandAudio>()
            .add_audio_channel::<ResetAudio>()
            .add_startup_system(setup_player)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(pause_run_criteria)
                    .with_system(reset_player.before(Labels::Contacts))
                    .with_system(compute_contacts.label(Labels::Contacts))
                    .with_system(
                        move_player
                            .after(Labels::Contacts)
                            .label(Labels::Kinematics),
                    )
                    .with_system(
                        player_crouch
                            .after(Labels::Contacts)
                            .label(Labels::Kinematics),
                    )
                    .with_system(player_audio.after(Labels::Contacts))
                    .with_system(player_dump_kinematics.after(Labels::Kinematics)),
            );
    }
}
