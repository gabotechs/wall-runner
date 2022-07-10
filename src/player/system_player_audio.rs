use crate::player::PlayerState;
use crate::{AssetServer, Res, ResMut};
use bevy_kira_audio::*;

pub struct WallRunAudio;

pub struct LandAudio;

const CONSIDER_PLAYED_LANDED_AFTER_FRAMES: u8 = 10;

#[derive(Default)]
pub struct PlayerAudioState {
    is_playing_wall_running: bool,
    play_land_when_grounded_ttl: u128,
}

pub fn player_audio(
    player_state: Res<PlayerState>,
    mut player_audio_state: ResMut<PlayerAudioState>,
    wall_run_audio: Res<AudioChannel<WallRunAudio>>,
    land_audio: Res<AudioChannel<LandAudio>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(wall_running) = &player_state.wall_running {
        if wall_running.just_started {
            player_audio_state.is_playing_wall_running = true;
            wall_run_audio.stop();
            wall_run_audio.play(asset_server.load("wall-run.flac"));
        }
    } else if player_audio_state.is_playing_wall_running {
        wall_run_audio.stop();
        wall_run_audio.play(asset_server.load("wall-run-decay.wav"));
        player_audio_state.is_playing_wall_running = false;
    }
    if !player_state.is_in_ground {
        player_audio_state.play_land_when_grounded_ttl += 1;
    } else if player_audio_state.play_land_when_grounded_ttl
        > CONSIDER_PLAYED_LANDED_AFTER_FRAMES as u128
    {
        land_audio.play(asset_server.load("land.flac"));
        player_audio_state.play_land_when_grounded_ttl = 0;
    } else {
        player_audio_state.play_land_when_grounded_ttl = 0;
    }
}
