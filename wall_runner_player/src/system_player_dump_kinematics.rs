use crate::component_player_kinematics::{HorizontalDisplacement, PlayerKinematics};
use crate::component_player_state::PlayerState;
use crate::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{ExternalForce, GravityScale, Velocity};

pub fn player_dump_kinematics(
    mut player_query: Query<
        (
            &Transform,
            &mut Velocity,
            &mut ExternalForce,
            &mut GravityScale,
            &mut PlayerState,
            &mut PlayerKinematics,
        ),
        With<Player>,
    >,
) {
    for (&transform, mut velocity, mut force, mut gravity, mut state, mut kinematics) in
        player_query.iter_mut()
    {
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
        // always dump gravity
        gravity.0 = kinematics.gravity;

        // update wall_runner_player state
        state.velocity = *velocity;
        state.position = transform.translation;
        // reset the kinematics, the next frame will be a new day
        *kinematics = PlayerKinematics::default();
    }
}
