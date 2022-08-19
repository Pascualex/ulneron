use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct InputEvent {
    pub id: Uuid,
    pub direction: Vec2,
}

impl InputEvent {
    pub fn new(id: Uuid, direction: Vec2) -> Self {
        Self { id, direction }
    }
}
