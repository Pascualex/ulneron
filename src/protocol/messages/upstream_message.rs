use serde::{Deserialize, Serialize};

use crate::protocol::events::UpstreamEvent;

#[derive(Serialize, Deserialize)]
pub struct UpstreamMessage {
    pub event: UpstreamEvent,
    pub rollback: Option<u32>,
}

impl UpstreamMessage {
    pub fn new(event: UpstreamEvent, rollback: Option<u32>) -> Self {
        Self { event, rollback }
    }
}
