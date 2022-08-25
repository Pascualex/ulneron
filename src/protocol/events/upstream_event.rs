use crate::protocol::data::Action;

pub struct UpstreamEvent {
    pub player_id: u32,
    pub action: Action,
}

impl UpstreamEvent {
    pub fn new(player_id: u32, action: Action) -> Self {
        Self { player_id, action }
    }

    pub fn new_local(action: Action) -> Self {
        Self {
            player_id: 0,
            action,
        }
    }
}
