use uuid::Uuid;

use crate::protocol::data::Action;

#[derive(Default)]
pub struct PlayersInfo {
    pub vec: Vec<PlayerInfo>,
}

pub struct PlayerInfo {
    pub uuid: Uuid,
    pub action: Action,
}

impl PlayerInfo {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid,
            action: Action::new(),
        }
    }
}
