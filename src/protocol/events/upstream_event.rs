use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::protocol::data::Action;

#[derive(Clone, Serialize, Deserialize)]
pub struct UpstreamEvent {
    pub id: u8,
    pub data: UpstreamData,
}

impl UpstreamEvent {
    pub fn new_local(data: UpstreamData) -> Self {
        Self { id: 0, data }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UpstreamData {
    Join(Uuid),
    Action(Action),
}
