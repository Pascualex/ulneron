use bevy::prelude::*;
use uuid::Uuid;

#[derive(Default)]
pub struct PlayersInfo {
    pub vec: Vec<PlayerInfo>,
}

pub struct PlayerInfo {
    pub uuid: Uuid,
    pub entity: Entity,
}

impl PlayerInfo {
    pub fn new(uuid: Uuid, entity: Entity) -> Self {
        Self { uuid, entity }
    }
}
