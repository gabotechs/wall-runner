use super::entity_player::Player;
use super::pre_update_system_player_contacts::player_contacts;
use super::resource_player_settings::PlayerSettings;
use super::resource_player_state::PlayerState;
use super::update_system_player_jump::jump_player;
use super::update_system_player_move::move_player;
use crate::player::update_system_player_gravity::update_gravity;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn update_position(
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
        .insert(Collider::cuboid(
            player_settings.width / 2.0,
            player_settings.height / 2.0,
            player_settings.width / 2.0,
        ))
        .insert(Velocity::default())
        .insert(RigidBody::Dynamic)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(ExternalForce::default())
        .insert(GravityScale::default())
        .insert(Player);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .init_resource::<PlayerSettings>()
            .add_startup_system(setup_player)
            .add_system_to_stage(CoreStage::PreUpdate, player_contacts)
            .add_system(update_gravity)
            .add_system(update_position)
            .add_system(jump_player)
            .add_system(move_player);
    }
}
