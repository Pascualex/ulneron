use bevy::utils::HashMap;

use crate::protocol::data::Tick;

pub struct DownstreamBuffer {
    pub sequence_number: u32,
    pub ticks_sent: u32,
    pub ticks: HashMap<u32, Tick>,
    pub patience: u32,
}

impl DownstreamBuffer {
    pub fn new() -> Self {
        Self {
            sequence_number: 0,
            ticks_sent: 0,
            ticks: HashMap::new(),
            patience: 5,
        }
    }
}
