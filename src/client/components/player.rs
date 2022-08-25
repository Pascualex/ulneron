use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component)]
pub struct Player {
    pub id: Uuid,
}

impl Player {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
