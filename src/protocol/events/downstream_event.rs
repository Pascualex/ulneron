use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DownstreamEvent {
    pub direction: Vec2,
}

impl DownstreamEvent {
    pub fn new(direction: Vec2) -> Self {
        Self { direction }
    }
}
