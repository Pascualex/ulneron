use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Velocity {
    pub val: Vec2,
}

impl Velocity {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            val: Vec2::new(x, y),
        }
    }
}
