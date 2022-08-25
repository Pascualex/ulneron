use bevy::prelude::*;

use crate::client::{
    components::{Player, Position, Velocity},
    resources::{PlayerIds, TickBuffer},
};

pub fn spawn(
    tick_buffer: Res<TickBuffer>,
    mut player_ids: ResMut<PlayerIds>,
    mut commands: Commands,
) {
    let tick = match tick_buffer.ticks.first() {
        Some(tick) => tick,
        None => return,
    };

    for id in tick.keys() {
        if player_ids.map.contains_key(id) {
            continue;
        }
        let entity = commands
            .spawn()
            .insert(Position::from_xy(0.0, 0.0))
            .insert(Velocity::from_xy(0.0, 0.0))
            .insert(Player)
            .id();
        player_ids.map.try_insert(*id, entity).unwrap();
    }
}
