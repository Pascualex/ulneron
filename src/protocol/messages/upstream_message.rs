use serde::{Deserialize, Serialize};

use crate::protocol::data::Action;

#[derive(Serialize, Deserialize)]
pub struct UpstreamMessage {
    pub action: Action,
    pub rollback: Option<u32>,
}

impl UpstreamMessage {
    pub fn new(action: Action, rollback: Option<u32>) -> Self {
        Self { action, rollback }
    }
}
