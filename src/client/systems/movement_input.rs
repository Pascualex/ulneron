use bevy::prelude::*;

use crate::client::{
    components::{Player, Velocity},
    resources::{PlayerIds, TickBuffer},
};

pub fn movement_input(
    tick_buffer: Res<TickBuffer>,
    player_ids: Res<PlayerIds>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let tick = match tick_buffer.ticks.first() {
        Some(tick) => tick,
        None => return,
    };

    for (id, action) in tick.iter() {
        let entity = *player_ids.map.get(id).unwrap();
        let mut velocity = query.get_mut(entity).unwrap();
        velocity.value = action.direction * 5.0;
    }
}
