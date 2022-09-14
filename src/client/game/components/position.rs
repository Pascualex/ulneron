use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub val: Vec2,
}

impl Position {
    pub fn new(val: Vec2) -> Self {
        Self { val }
    }

    pub fn from_xy(x: f32, y: f32) -> Self {
        Self::new(Vec2::new(x, y))
    }
}
