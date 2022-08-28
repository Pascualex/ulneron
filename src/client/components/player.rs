use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: u8,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}
