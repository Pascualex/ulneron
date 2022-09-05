use bevy::prelude::*;

use crate::client::game::{
    components::{Player, Velocity},
    resources::{PlayerEntities, Ticks},
};

pub fn movement_action(
    ticks: Res<Ticks>,
    player_entities: Res<PlayerEntities>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let tick = match ticks.current() {
        Some(tick) => tick,
        None => return,
    };

    for (id, action) in tick.actions.iter().enumerate() {
        let entity = player_entities.vec[id];
        let mut velocity = query.get_mut(entity).unwrap();
        velocity.value = action.direction.normalize_or_zero() * 5.0;
    }
}
