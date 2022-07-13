use bevy::prelude::*;

#[derive(Copy, Clone)]
pub enum HorizontalDisplacement {
    Velocity(Vec2),
    Force(Vec2),
    None,
}

use HorizontalDisplacement::*;

impl HorizontalDisplacement {
    pub fn add_velocity(&mut self, v: Vec2) {
        match self {
            None => {
                *self = Velocity(v);
            }
            Velocity(prev_v) => {
                *prev_v += v;
            }
            Force(_) => {
                error!("Tried to add velocity to a kinematic displacement of type Force");
            }
        }
    }

    pub fn add_force(&mut self, v: Vec2) {
        match self {
            None => {
                *self = Force(v);
            }
            Force(prev_v) => {
                *prev_v += v;
            }
            Velocity(_) => {
                error!("Tried to add a force to a kinematic displacement of type Velocity");
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct PlayerKinematics {
    pub displacement: HorizontalDisplacement,
    pub vertical_impulse: f32,
    pub gravity: f32,
}

impl Default for PlayerKinematics {
    fn default() -> Self {
        PlayerKinematics {
            displacement: None,
            vertical_impulse: 0.0,
            gravity: 1.0,
        }
    }
}
