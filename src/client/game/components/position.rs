use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub val: Vec2,
}

impl Position {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            val: Vec2::new(x, y),
        }
    }
}
