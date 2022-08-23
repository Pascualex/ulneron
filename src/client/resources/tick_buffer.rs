use crate::protocol::data::Tick;

#[derive(Default)]
pub struct TickBuffer {
    pub ticks: Vec<Tick>,
}
