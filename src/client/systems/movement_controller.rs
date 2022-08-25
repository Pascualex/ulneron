use bevy::prelude::*;

use crate::client::{
    components::{Player, Velocity},
    resources::{PlayerEntities, TickBuffer},
};

pub fn movement_controller(
    tick_buffer: Res<TickBuffer>,
    player_ids: Res<PlayerEntities>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let tick = match tick_buffer.ticks.first() {
        Some(tick) => tick,
        None => return,
    };

    for (id, action) in tick.iter() {
        let entity = *player_ids.map.get(id).unwrap();
        let mut velocity = query.get_mut(entity).unwrap();
        velocity.value = action.direction.normalize_or_zero() * 5.0;
    }
}
