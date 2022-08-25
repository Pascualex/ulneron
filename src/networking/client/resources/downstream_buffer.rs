use bevy::utils::HashMap;

use crate::protocol::data::Tick;

#[derive(Default)]
pub struct DownstreamBuffer {
    pub current: u32,
    pub ticks: HashMap<u32, Tick>,
}

impl DownstreamBuffer {
    pub fn current(&self) -> Option<&Tick> {
        self.ticks.get(&self.current)
    }
}
