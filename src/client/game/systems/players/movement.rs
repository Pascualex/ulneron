use bevy::prelude::*;

use crate::client::game::{
    components::{Player, Velocity},
    resources::Ticks,
};

pub fn players_movement(ticks: Res<Ticks>, mut query: Query<(&mut Velocity, &Player)>) {
    let tick = match &ticks.current {
        Some(tick) => tick,
        None => return,
    };

    for (mut velocity, player) in query.iter_mut() {
        let action = &tick.actions[player.id];
        velocity.val = action.direction.normalize_or_zero() * 5.0;
    }
}
