use bevy::prelude::*;

pub fn vec3_horizontal_vec2(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.z)
}
