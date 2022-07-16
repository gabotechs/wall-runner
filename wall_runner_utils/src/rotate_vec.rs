use bevy::prelude::*;

pub fn rotate_vec(v: Vec2, angle: f32) -> Vec2 {
    Vec2::new(
        v.x * angle.cos() - v.y * angle.sin(),
        v.x * angle.sin() + v.y * angle.cos(),
    )
}

#[cfg(test)]
mod tests {
    use crate::rotate_vec;
    use crate::{assert_almost_eq};
    use bevy::prelude::*;
    use std::f32::consts::PI;

    #[test]
    fn test_rotate_clockwise_vec2() {
        let input = Vec2::new(1.0, 0.0);
        let rotated = rotate_vec(input, PI / 2.0);
        assert_almost_eq!(rotated.x, 0.0);
        assert_almost_eq!(rotated.y, 1.0);
    }

    #[test]
    fn test_rotate_counter_clockwise_vec2() {
        let input = Vec2::new(1.0, 0.0);
        let rotated = rotate_vec(input, -PI / 2.0);
        assert_almost_eq!(rotated.x, 0.0);
        assert_almost_eq!(rotated.y, -1.0);
    }
}
