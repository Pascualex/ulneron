use bevy::prelude::*;
use uuid::Uuid;

#[derive(Default)]
pub struct PlayersInfo {
    pub uuids: Vec<Uuid>,
    pub entities: Vec<Entity>,
}
