use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Agent {
    pub preferred_velocity: Vec2,
}

impl Agent {
    pub fn new() -> Self {
        Self::default()
    }
}
