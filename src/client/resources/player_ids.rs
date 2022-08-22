use bevy::prelude::*;
use bevy::utils::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct PlayerIds {
    pub map: HashMap<Uuid, Entity>,
}
