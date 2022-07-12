use super::Player;
use crate::player::{PlayerSettings, PlayerState, WallRunningState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::borrow::BorrowMut;

fn rotate(v: Vec2, angle: f32) -> Vec2 {
    Vec2::new(
        v.x * angle.cos() - v.y * angle.sin(),
        v.x * angle.sin() + v.y * angle.cos(),
    )
}

pub fn player_contacts(
    rapier_context: Res<RapierContext>,
    settings: Res<PlayerSettings>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<Entity, With<Player>>,
) {
    let mut normal_force = Vec3::default();
    for entity in player_query.iter_mut() {
        for contact_pair in rapier_context.contacts_with(entity) {
            let direction = if contact_pair.collider1() == entity {
                -1.0
            } else {
                1.0
            };
            let best_manifold = contact_pair.find_deepest_contact();
            if let Some((manifold, _)) = best_manifold {
                normal_force = manifold.normal() * direction;
            }
        }
    }
    let has_vertical_forces = normal_force.y > 0.9;
    let has_horizontal_forces = (normal_force.x.powi(2) + normal_force.z.powi(2)).sqrt() > 0.9;
    // if the manifold is positive and vertical, then we are on the ground
    player_state.is_in_ground = false;
    if has_vertical_forces {
        player_state.is_in_ground = true;
        // if the sum vector of all the normal forces has mainly an horizontal component, then we are wall running
    } else if has_horizontal_forces && player_state.wall_running.is_none() {
        if player_state.wall_run_vote < settings.wall_run_votes {
            player_state.wall_run_vote += 1;
            info!("[+] vote wall run {}", player_state.wall_run_vote);
            return;
        }
        let current_direction = player_state.kinematics.displacement;
        // clockwise
        let v1 = Vec2::new(-normal_force.z, normal_force.x);
        // counter-clockwise
        let v2 = Vec2::new(normal_force.z, -normal_force.x);
        const ANGLE_EPSILON: f32 = 0.1;
        let tangent_direction = if v1.angle_between(current_direction).abs()
            < v2.angle_between(current_direction).abs()
        {
            rotate(v1, ANGLE_EPSILON)
        } else {
            rotate(v2, -ANGLE_EPSILON)
        };
        info!("start wall run,  direction: {:?}", tangent_direction);
        player_state.wall_running = Some(WallRunningState {
            speed: None,
            just_started: true,
            normal_force,
            direction: tangent_direction.normalize_or_zero(),
        });
    } else if !has_horizontal_forces && player_state.wall_running.is_some() {
        if player_state.wall_run_vote > 0 {
            player_state.wall_run_vote -= 2;
            info!("[-] vote wall run {}", player_state.wall_run_vote);
            return;
        }
        info!("stop wall run");
        player_state.wall_running = None;
    } else if let Some(wall_running) = player_state.wall_running.borrow_mut() {
        wall_running.just_started = false;
    }
}

#[cfg(test)]
mod tests {
    use crate::player::pre_system_player_contacts::rotate;
    use crate::Vec2;
    use std::f32::consts::PI;

    macro_rules! assert_almost_eq {
        ($n1: expr, $n2: expr) => {
            match (&$n1, &$n2) {
                (n1, n2) => {
                    if (n1 - n2).abs() > 0.0000001 {
                        assert_eq!(n1, n2)
                    }
                }
            }
        };
    }

    #[test]
    fn test_rotate_clockwise_vec2() {
        let input = Vec2::new(1.0, 0.0);
        let rotated = rotate(input, PI / 2.0);
        assert_almost_eq!(rotated.x, 0.0);
        assert_almost_eq!(rotated.y, 1.0);
    }

    #[test]
    fn test_rotate_counter_clockwise_vec2() {
        let input = Vec2::new(1.0, 0.0);
        let rotated = rotate(input, -PI / 2.0);
        assert_almost_eq!(rotated.x, 0.0);
        assert_almost_eq!(rotated.y, -1.0);
    }
}
