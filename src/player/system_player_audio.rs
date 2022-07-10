use crate::player::PlayerState;
use crate::{AssetServer, Res, ResMut, Time};
use bevy_kira_audio::*;

pub struct WallRunAudio;

pub struct LandAudio;

const PLAYED_LANDED_AFTER_MS: u128 = 1000;

#[derive(Default)]
pub struct PlayerAudioState {
    is_playing_wall_running: bool,
    in_air_since: u128,
}

pub fn player_audio(
    player_state: Res<PlayerState>,
    time: Res<Time>,
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
    let now = time.time_since_startup().as_millis();
    if player_audio_state.in_air_since == 0 {
        player_audio_state.in_air_since = now;
    }
    if player_state.is_in_ground {
        if (now - player_audio_state.in_air_since) > PLAYED_LANDED_AFTER_MS {
            land_audio.play(asset_server.load("land.flac"));
        }
        player_audio_state.in_air_since = now;
    }
}
