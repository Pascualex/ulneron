use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Default)]
pub struct PlayerEntities {
    pub map: HashMap<u32, Entity>,
}
