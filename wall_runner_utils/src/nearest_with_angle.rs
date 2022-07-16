use crate::rotate_vec;
use bevy::prelude::*;

pub fn nearest_with_angle(v: Vec2, other: Vec2, angle: f32) -> Vec2 {
    // clockwise
    let v1 = rotate_vec(v, angle);
    // counter-clockwise
    let v2 = rotate_vec(v, -angle);
    if v1.angle_between(other).abs() < v2.angle_between(other).abs() {
        v1
    } else {
        v2
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;
    use std::f32::consts::PI;
    use crate::{assert_almost_eq, nearest_with_angle};

    #[test]
    fn test_nearest_with_angle() {
        let input = Vec2::new(1.0, 0.0);
        let base = Vec2::new(-(1.0_f32.sqrt()), 1.0_f32.sqrt());
        let result = nearest_with_angle(input, base, PI / 2.0);
        assert_almost_eq!(result.x, 0.0);
        assert_almost_eq!(result.y, 1.0);
    }
}
