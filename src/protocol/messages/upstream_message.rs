use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::protocol::data::Action;

#[derive(Serialize, Deserialize)]
pub struct UpstreamMessage {
    pub id: Uuid,
    pub action: Action,
}

impl UpstreamMessage {
    pub fn new(id: Uuid, action: Action) -> Self {
        Self { id, action }
    }
}
