use super::player_entity::Player;
use super::player_settings::PlayerSettings;
use super::player_state::PlayerState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    player_state: Res<PlayerState>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    let angle = player_state.y_angle;
    for mut velocity in player_query.iter_mut() {
        let mut vec = Vec3::new(0.0, velocity.linvel.y, 0.0);
        let forward_speed = if keys.pressed(settings.run) {
            settings.run_speed
        } else {
            settings.walk_speed
        };
        let backward_speed = settings.walk_speed;
        let lateral_speed = settings.walk_speed * 0.5;
        for key in keys.get_pressed() {
            if key == &settings.forward {
                vec += Vec3::new(
                    -forward_speed * angle.sin(),
                    0.0,
                    -forward_speed * angle.cos(),
                );
            } else if key == &settings.backward {
                vec += Vec3::new(
                    backward_speed * angle.sin(),
                    0.0,
                    backward_speed * angle.cos(),
                );
            } else if key == &settings.left {
                vec += Vec3::new(
                    -lateral_speed * angle.cos(),
                    0.0,
                    lateral_speed * angle.sin(),
                );
            } else if key == &settings.right {
                vec += Vec3::new(
                    lateral_speed * angle.cos(),
                    0.0,
                    -lateral_speed * angle.sin(),
                );
            }
        }
        velocity.linvel = vec;
    }
}
