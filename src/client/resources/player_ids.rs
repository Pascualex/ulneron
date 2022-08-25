use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Default)]
pub struct PlayerIds {
    pub map: HashMap<u32, Entity>,
}
