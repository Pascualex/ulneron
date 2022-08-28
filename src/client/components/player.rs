use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: usize,
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}
