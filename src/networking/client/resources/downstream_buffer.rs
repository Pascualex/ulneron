use bevy::utils::HashMap;

use crate::protocol::data::Tick;

pub struct DownstreamBuffer {
    pub sequence_number: u32,
    pub ticks: HashMap<u32, Tick>,
}

impl DownstreamBuffer {
    pub fn new() -> Self {
        Self {
            sequence_number: 0,
            ticks: HashMap::new(),
        }
    }

    pub fn current(&self) -> Option<&Tick> {
        self.ticks.get(&self.sequence_number)
    }
}
