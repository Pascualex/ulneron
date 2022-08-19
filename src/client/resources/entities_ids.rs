use bevy::prelude::*;
use bevy::utils::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct EntitiesIds {
    pub map: HashMap<Uuid, Entity>,
}

impl EntitiesIds {
    pub fn new() -> Self {
        Self::default()
    }
}
