use bevy::prelude::*;

use crate::{
    client::game::resources::{GameState, Ticks},
    protocol::events::GameDownstreamEvent,
};

pub fn tick(
    mut reader: EventReader<GameDownstreamEvent>,
    state: Res<GameState>,
    mut ticks: ResMut<Ticks>,
) {
    ticks.vec.extend(reader.iter().map(|e| e.tick.clone()));
    if matches!(*state, GameState::Running) {
        ticks.current = match ticks.vec.len() {
            0 => None,
            _ => Some(ticks.vec.remove(0)),
        }
    }
}
