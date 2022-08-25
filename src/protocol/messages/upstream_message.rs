use serde::{Deserialize, Serialize};

use crate::protocol::data::Action;

#[derive(Serialize, Deserialize)]
pub struct UpstreamMessage {
    pub action: Action,
}

impl UpstreamMessage {
    pub fn new(action: Action) -> Self {
        Self { action }
    }
}
