use std::time::Duration;

use bevy::prelude::*;

use crate::{
    client::game::resources::Ticks,
    client::{game::resources::GameState, graphics::resources::TickDelta},
};

pub fn tick_delta(
    state: Res<GameState>,
    ticks: Res<Ticks>,
    time: Res<Time>,
    mut tick_delta: ResMut<TickDelta>,
) {
    tick_delta.delta = match matches!(*state, GameState::Running) && ticks.current.is_none() {
        true => tick_delta.delta + time.delta(),
        false => Duration::ZERO,
    };
}
