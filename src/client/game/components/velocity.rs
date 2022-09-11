use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub val: Vec2,
}

impl Velocity {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            val: Vec2::new(x, y),
        }
    }
}
