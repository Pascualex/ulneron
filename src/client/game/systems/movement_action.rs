use bevy::prelude::*;

use crate::client::game::{
    components::{Player, Velocity},
    resources::{PlayersInfo, Ticks},
};

pub fn movement_action(
    ticks: Res<Ticks>,
    players_info: Res<PlayersInfo>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let tick = match ticks.current() {
        Some(tick) => tick,
        None => return,
    };

    for (id, action) in tick.iter().enumerate() {
        let entity = players_info.entities[id];
        let mut velocity = query.get_mut(entity).unwrap();
        velocity.value = action.direction.normalize_or_zero() * 5.0;
    }
}
