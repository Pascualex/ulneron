use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MovementEvent {
    pub position: Vec2,
    pub velocity: Vec2,
}
