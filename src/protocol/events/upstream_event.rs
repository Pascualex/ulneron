use uuid::Uuid;

use crate::protocol::data::Action;

pub struct UpstreamEvent {
    pub id: Uuid,
    pub action: Action,
}

impl UpstreamEvent {
    pub fn new(id: Uuid, action: Action) -> Self {
        Self { id, action }
    }
}
