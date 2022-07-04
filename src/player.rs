use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct MoveSettings {
    pub speed: f32,
    pub jump_velocity: f32,
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub run: KeyCode,
}

impl Default for MoveSettings {
    fn default() -> Self {
        MoveSettings {
            speed: 10.0,
            jump_velocity: 10.0,
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
            run: KeyCode::LShift,
        }
    }
}

pub struct PlayerSettings {
    size: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings { size: 1.0 }
    }
}

#[derive(Default)]
pub struct PlayerPosition(pub f32, pub f32, pub f32);

#[derive(Default)]
pub struct DisplacementAngle(pub f32);

#[derive(Component)]
pub struct Player;

fn jump_player(
    keys: Res<Input<KeyCode>>,
    player_settings: Res<PlayerSettings>,
    settings: Res<MoveSettings>,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(&Transform, &mut Velocity, &Collider, Entity), With<Player>>,
) {
    for (transform, mut velocity, collider, entity) in player_query.iter_mut() {
        if keys.just_pressed(settings.jump) {
            let ray_pos = Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            );
            let ray_dir = Vec3::new(0.0, -1.0, 0.0);
            let max_toi = player_settings.size / 2.0;
            let groups = InteractionGroups::all();
            if rapier_context
                .cast_shape(
                    ray_pos,
                    Quat::default(),
                    ray_dir,
                    collider,
                    max_toi,
                    groups,
                    Some(&|en: Entity| entity != en),
                )
                .is_some()
            {
                velocity.linvel += Vec3::new(0.0, settings.jump_velocity, 0.0);
            }
        }
    }
}

fn move_player(
    keys: Res<Input<KeyCode>>,
    settings: Res<MoveSettings>,
    angle: Res<DisplacementAngle>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
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
                    -forward_speed * angle.0.sin(),
                    0.0,
                    -forward_speed * angle.0.cos(),
                );
            } else if key == &settings.backward {
                vec += Vec3::new(
                    backward_speed * angle.0.sin(),
                    0.0,
                    backward_speed * angle.0.cos(),
                );
            } else if key == &settings.left {
                vec += Vec3::new(
                    -lateral_speed * angle.0.cos(),
                    0.0,
                    lateral_speed * angle.0.sin(),
                );
            } else if key == &settings.right {
                vec += Vec3::new(
                    lateral_speed * angle.0.cos(),
                    0.0,
                    -lateral_speed * angle.0.sin(),
                );
            }
        }
        velocity.linvel = vec;
    }
}

fn update_position(
    mut position: ResMut<PlayerPosition>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    for transform in player_query.iter_mut() {
        position.0 = transform.translation.x;
        position.1 = transform.translation.y;
        position.2 = transform.translation.z;
    }
}

fn setup_player(
    mut commands: Commands,
    position: Res<PlayerPosition>,
    player_settings: Res<PlayerSettings>,
) {
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            position.0, position.1, position.2,
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
        app.init_resource::<MoveSettings>()
            .init_resource::<PlayerSettings>()
            .init_resource::<PlayerPosition>()
            .init_resource::<DisplacementAngle>()
            .add_startup_system(setup_player)
            .add_system(update_position)
            .add_system(jump_player)
            .add_system(move_player);
    }
}
