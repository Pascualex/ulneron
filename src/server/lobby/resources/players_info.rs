use bevy::prelude::*;
use uuid::Uuid;

#[derive(Default, Resource)]
pub struct PlayersInfo {
    pub uuids: Vec<Uuid>,
}
