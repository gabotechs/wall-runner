use crate::player::resource_player_kinematics::{HorizontalDisplacement, PlayerKinematics};
use crate::player::{Player, PlayerState};
use bevy::prelude::*;
use bevy_rapier3d::dynamics::GravityScale;
use bevy_rapier3d::prelude::{ExternalForce, Velocity};

pub fn player_dump_kinematics(
    mut kinematics: ResMut<PlayerKinematics>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<
        (
            &Transform,
            &mut Velocity,
            &mut ExternalForce,
            &mut GravityScale,
        ),
        With<Player>,
    >,
) {
    for (&transform, mut velocity, mut force, mut gravity) in player_query.iter_mut() {
        velocity.linvel.y += kinematics.vertical_impulse;
        match kinematics.displacement {
            HorizontalDisplacement::Force(v) => {
                // force controlled displacement, let the velocity run free
                force.force = Vec3::new(v.x, 0.0, v.y);
            }
            HorizontalDisplacement::Velocity(v) => {
                // velocity controlled displacement, we decide it
                velocity.linvel.x = v.x;
                velocity.linvel.z = v.y;
                force.force = Vec3::default();
            }
            HorizontalDisplacement::None => {
                // do nothing
            }
        }
        gravity.0 = kinematics.gravity;

        // update player state
        player_state.velocity = *velocity;
        player_state.position = transform.translation;
    }
    *kinematics = PlayerKinematics::default();
}
