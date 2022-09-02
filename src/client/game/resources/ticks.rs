use crate::protocol::data::Tick;

#[derive(Default)]
pub struct Ticks {
    pub vec: Vec<Tick>,
    pub current: Option<Tick>,
}

impl Ticks {
    pub fn current(&self) -> &Option<Tick> {
        &self.current
    }
}
