use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct MovementEvent {
    pub id: Uuid,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl MovementEvent {
    pub fn new(id: Uuid, position: Vec2, velocity: Vec2) -> Self {
        Self {
            id,
            position,
            velocity,
        }
    }
}
