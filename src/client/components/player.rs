use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: u32,
}

impl Player {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
