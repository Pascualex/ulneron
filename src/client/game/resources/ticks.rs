use crate::server::game::data::Tick;

#[derive(Default)]
pub struct Ticks {
    pub vec: Vec<Tick>,
    pub current: Option<Tick>,
}
