use bevy::prelude::*;

use crate::client::game::{
    components::{Agent, Player, Stats},
    resources::Ticks,
};

pub fn players_pathfinder(ticks: Res<Ticks>, mut query: Query<(&mut Agent, &Stats, &Player)>) {
    let tick = match &ticks.current {
        Some(tick) => tick,
        None => return,
    };

    for (mut agent, stats, player) in query.iter_mut() {
        let action = &tick.actions[player.id];
        agent.preferred_velocity = action.direction.normalize_or_zero() * stats.speed;
    }
}
