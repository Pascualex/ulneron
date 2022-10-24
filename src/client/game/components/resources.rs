use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Resources {
    pub nerite: u32,
    pub kills: u32,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }
}
