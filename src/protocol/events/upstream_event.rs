use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UpstreamEvent {
    pub direction: Vec2,
}

impl UpstreamEvent {
    pub fn new(direction: Vec2) -> Self {
        Self { direction }
    }
}
