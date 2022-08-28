use bevy::prelude::*;

use crate::client::{
    components::{Player, Velocity},
    resources::{PlayersInfo, Ticks},
};

pub fn movement_controller(
    ticks: Res<Ticks>,
    players_info: Res<PlayersInfo>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let tick = match ticks.current() {
        Some(tick) => tick,
        None => return,
    };

    for (id, action) in tick.iter().enumerate() {
        let entity = players_info.vec[id].entity;
        let mut velocity = query.get_mut(entity).unwrap();
        velocity.value = action.direction.normalize_or_zero() * 5.0;
    }
}
