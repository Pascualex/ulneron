use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::protocol::data::Action;

#[derive(Clone, Serialize, Deserialize)]
pub struct UpstreamEvent {
    pub id: Uuid,
    pub action: Action,
}

impl UpstreamEvent {
    pub fn new(id: Uuid, action: Action) -> Self {
        Self { id, action }
    }
}
