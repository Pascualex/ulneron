use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::protocol::data::Action;

#[derive(Serialize, Deserialize)]
pub struct UpstreamMessage {
    pub id: Uuid,
    pub action: Action,
    pub rollback: Option<u32>,
}

impl UpstreamMessage {
    pub fn new(id: Uuid, action: Action, rollback: Option<u32>) -> Self {
        Self {
            id,
            action,
            rollback,
        }
    }
}
