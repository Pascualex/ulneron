use bevy::prelude::*;
use bevy::utils::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct PlayerEntities {
    pub map: HashMap<Uuid, Entity>,
}
