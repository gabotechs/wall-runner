use crate::player::{Player, PlayerInput};
use bevy::prelude::*;

pub fn player_apply_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    player_input: Res<PlayerInput>,
) {
    for mut transform in player_query.iter_mut() {
        transform.rotation = Quat::from_axis_angle(Vec3::Y, player_input.y_angle);
    }
}
