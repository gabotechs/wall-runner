use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerSettings {
    pub size: f32,
    pub speed: f32,
    pub jump_velocity: f32,
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub run: KeyCode,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings {
            size: 2.0,
            speed: 10.0,
            jump_velocity: 5.0,
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
            run: KeyCode::LShift,
        }
    }
}

pub struct PlayerState {
    pub position: Vec3,
    pub y_angle: f32,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            position: Vec3::new(0.0, 0.0, 0.0),
            y_angle: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Player;

fn is_in_ground(
    entity: Entity,
    player_size: f32,
    current_position: Vec3,
    rapier_context: &Res<RapierContext>,
) -> bool {
    let ray_dir = Vec3::new(0.0, -1.0, 0.0);
    let groups = InteractionGroups::all();
    rapier_context
        .cast_shape(
            current_position,
            Quat::default(),
            ray_dir,
            &Collider::ball(player_size),
            player_size,
            groups,
            Some(&|en: Entity| entity != en),
        )
        .is_some()
}

fn jump_player(
    keys: Res<Input<KeyCode>>,
    player_settings: Res<PlayerSettings>,
    settings: Res<PlayerSettings>,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(&Transform, &mut Velocity, Entity), With<Player>>,
) {
    for (transform, mut velocity, entity) in player_query.iter_mut() {
        if keys.just_pressed(settings.jump) {
            let is_in_ground = is_in_ground(
                entity,
                player_settings.size / 2.0,
                Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                ),
                &rapier_context,
            );
            if is_in_ground {
                velocity.linvel += Vec3::new(0.0, settings.jump_velocity, 0.0);
            }
        }
    }
}

fn move_player(
    keys: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    player_state: Res<PlayerState>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    let angle = player_state.y_angle;
    for mut velocity in player_query.iter_mut() {
        let mut vec = Vec3::new(0.0, velocity.linvel.y, 0.0);
        let mut forward_speed = settings.speed;
        if keys.pressed(settings.run) {
            forward_speed *= 1.3;
        }
        let backward_speed = settings.speed;
        let lateral_speed = settings.speed * 0.5;
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

fn update_position(
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    for transform in player_query.iter_mut() {
        player_state.position = transform.translation;
    }
}

fn setup_player(
    mut commands: Commands,
    player_settings: Res<PlayerSettings>,
    player_state: Res<PlayerState>,
) {
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            player_state.position.x,
            player_state.position.y,
            player_state.position.z,
        )))
        .insert(Collider::ball(player_settings.size / 2.0))
        .insert(Velocity::default())
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Player);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .init_resource::<PlayerSettings>()
            .add_startup_system(setup_player)
            .add_system(update_position)
            .add_system(jump_player)
            .add_system(move_player);
    }
}
