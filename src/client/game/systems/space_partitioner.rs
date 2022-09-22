use bevy::prelude::*;

use crate::client::game::{
    components::{Agent, Enemy, Player, Position},
    resources::SpacePartitioner,
};

pub fn space_partitioner(
    player_query: Query<(Entity, &Position), With<Player>>,
    enemy_query: Query<(Entity, &Position), With<Enemy>>,
    agent_query: Query<(Entity, &Position), With<Agent>>,
    mut space_partitioner: ResMut<SpacePartitioner>,
) {
    space_partitioner.reset();
    for (entity, position) in player_query.iter() {
        let position = position.val.as_ref();
        space_partitioner.players.add(position, entity).unwrap();
    }
    for (entity, position) in enemy_query.iter() {
        let position = position.val.as_ref();
        space_partitioner.enemies.add(position, entity).unwrap();
    }
    for (entity, position) in agent_query.iter() {
        let position = position.val.as_ref();
        space_partitioner.agents.add(position, entity).unwrap();
    }
}
