use crate::component_player_state::PlayerState;
use crate::{PlayerInput, PlayerSettings};
use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use bevy_rapier3d::prelude::*;

pub struct ResetAudio;

pub fn reset_player(
    mut player_query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut ExternalForce,
        &mut PlayerInput,
        &mut PlayerState,
    )>,
    _reset_audio: Res<AudioChannel<ResetAudio>>,
    settings: Res<PlayerSettings>,
    _asset_server: Res<AssetServer>,
) {
    for (mut transform, mut velocity, mut force, mut player_input, mut player_state) in
        player_query.iter_mut()
    {
        if player_input.reset {
            info!("resetting wall_runner_player");
            transform.translation = settings.initial_position;
            velocity.linvel = Vec3::default();
            force.force = Vec3::default();
            player_state.reset();
            player_input.reset = false;
            // reset_audio.play(asset_server.load("esther.wav"));
        }
    }
}
