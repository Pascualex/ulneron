use bevy::prelude::*;

use crate::server::game::data::Tick;

#[derive(Default, Resource)]
pub struct Ticks {
    pub vec: Vec<Tick>,
    pub current: Option<Tick>,
}
