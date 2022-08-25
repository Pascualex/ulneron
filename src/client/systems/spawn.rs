use bevy::prelude::*;

use crate::client::{
    components::{Player, Position, Velocity},
    resources::{PlayerEntities, Ticks},
};

pub fn spawn(
    ticks: Res<Ticks>,
    mut player_entities: ResMut<PlayerEntities>,
    mut commands: Commands,
) {
    let tick = match ticks.current() {
        Some(tick) => tick,
        None => return,
    };

    for id in tick.keys() {
        if player_entities.map.contains_key(id) {
            continue;
        }
        let entity = commands
            .spawn()
            .insert(Position::from_xy(0.0, 0.0))
            .insert(Velocity::from_xy(0.0, 0.0))
            .insert(Player::new(*id))
            .id();
        player_entities.map.try_insert(*id, entity).unwrap();
    }
}
