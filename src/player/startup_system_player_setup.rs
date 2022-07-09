use crate::player::{Player, PlayerSettings};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_player(mut commands: Commands, player_settings: Res<PlayerSettings>) {
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            player_settings.initial_position.x,
            player_settings.initial_position.y,
            player_settings.initial_position.z,
        )))
        .insert(Collider::capsule_y(
            player_settings.size,
            player_settings.size * 0.25,
        ))
        .insert(Velocity::default())
        .insert(RigidBody::Dynamic)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(ColliderMassProperties::Density(1.0))
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(GravityScale::default())
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Player);
}