use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct SpawnEvent {
    pub id: Uuid,
    pub position: Vec2,
}

impl SpawnEvent {
    pub fn new(id: Uuid, position: Vec2) -> Self {
        Self { id, position }
    }
}
