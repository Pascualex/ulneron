use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct DownstreamEvent {
    pub directions: HashMap<Uuid, Vec2>,
}

impl DownstreamEvent {
    pub fn new(directions: HashMap<Uuid, Vec2>) -> Self {
        Self { directions }
    }
}
