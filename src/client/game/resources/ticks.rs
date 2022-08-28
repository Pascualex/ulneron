use crate::protocol::data::Tick;

#[derive(Default)]
pub struct Ticks {
    pub vec: Vec<Tick>,
}

impl Ticks {
    pub fn current(&self) -> Option<&Tick> {
        self.vec.first()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}
