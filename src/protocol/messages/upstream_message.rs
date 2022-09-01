use serde::{Deserialize, Serialize};

use crate::protocol::events::ControllerEventData;

#[derive(Serialize, Deserialize)]
pub struct UpstreamMessage {
    pub data: ControllerEventData,
}

impl UpstreamMessage {
    pub fn new(data: ControllerEventData) -> Self {
        Self { data }
    }
}
