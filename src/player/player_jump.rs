use super::player_entity::Player;
use super::player_settings::PlayerSettings;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn is_in_ground(entity: Entity, rapier_context: &Res<RapierContext>) -> bool {
    for contact_pair in rapier_context.contacts_with(entity) {
        for manifold in contact_pair.manifolds() {
            let normal = manifold.normal();
            println!("{:?}", normal);
            if normal.y.abs() > 0.95 {
                return true;
            }
        }
    }
    false
}

pub fn jump_player(
    keys: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(&mut Velocity, Entity), With<Player>>,
) {
    for (mut velocity, entity) in player_query.iter_mut() {
        if keys.just_pressed(settings.jump) {
            let is_in_ground = is_in_ground(entity, &rapier_context);
            if is_in_ground {
                velocity.linvel += Vec3::new(0.0, settings.jump_velocity, 0.0);
            }
        }
    }
}
