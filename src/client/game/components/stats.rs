use bevy::prelude::*;

#[derive(Component)]
pub struct Stats {
    pub speed: f32,
}

impl Stats {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}
