use bevy::utils::HashMap;

use crate::protocol::events::DownstreamEvent;

pub struct DownstreamBuffer {
    pub sequence_number: u32,
    pub events_sent: u32,
    pub events: HashMap<u32, DownstreamEvent>,
    pub patience: u32,
}

impl DownstreamBuffer {
    pub fn new() -> Self {
        Self {
            sequence_number: 0,
            events_sent: 0,
            events: HashMap::new(),
            patience: 5,
        }
    }
}
