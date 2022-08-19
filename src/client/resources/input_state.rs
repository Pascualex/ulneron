use bevy::prelude::*;

pub struct InputState {
    pub previous_direction: Vec2,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            previous_direction: Vec2::ZERO,
        }
    }
}
