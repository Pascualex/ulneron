use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Action {
    pub direction: Vec2,
}

impl Action {
    pub fn new() -> Self {
        Self::default()
    }
}
