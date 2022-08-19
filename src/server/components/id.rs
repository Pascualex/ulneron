use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component)]
pub struct Id {
    pub value: Uuid,
}

impl Id {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }
}
