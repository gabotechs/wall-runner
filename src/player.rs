use bevy::prelude::*;

pub struct MoveSettings {
    pub speed: f32,
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

impl Default for MoveSettings {
    fn default() -> Self {
        MoveSettings {
            speed: 0.1,
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
        }
    }
}

#[derive(Default)]
pub struct PlayerPosition(pub f32, pub f32, pub f32);

#[derive(Default)]
pub struct DisplacementAngle(pub f32);

#[derive(Component)]
pub struct Player;

fn move_player(
    keys: Res<Input<KeyCode>>,
    settings: Res<MoveSettings>,
    angle: Res<DisplacementAngle>,
    mut position: ResMut<PlayerPosition>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    for key in keys.get_pressed() {
        if key == &settings.forward {
            position.0 -= settings.speed * angle.0.sin();
            position.2 -= settings.speed * angle.0.cos();
        } else if key == &settings.backward {
            position.0 += settings.speed * angle.0.sin();
            position.2 += settings.speed * angle.0.cos();
        } else if key == &settings.left {
            position.0 -= settings.speed * angle.0.cos();
            position.2 += settings.speed * angle.0.sin();
        } else if key == &settings.right {
            position.0 += settings.speed * angle.0.cos();
            position.2 -= settings.speed * angle.0.sin();
        }
    }
    for mut player in player_query.iter_mut() {
        player.translation.x = position.0;
        player.translation.y = position.2;
    }
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(Player);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MoveSettings>()
            .init_resource::<PlayerPosition>()
            .init_resource::<DisplacementAngle>()
            .add_startup_system(setup_player)
            .add_system(move_player);
    }
}
