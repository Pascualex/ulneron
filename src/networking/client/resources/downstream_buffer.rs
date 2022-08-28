use bevy::utils::HashMap;

use crate::protocol::data::DownstreamData;

pub struct DownstreamBuffer {
    pub sequence_number: u32,
    pub events_sent: u32,
    pub data: HashMap<u32, DownstreamData>,
    pub patience: u32,
}

impl DownstreamBuffer {
    pub fn new() -> Self {
        Self {
            sequence_number: 0,
            events_sent: 0,
            data: HashMap::new(),
            patience: 5,
        }
    }
}
