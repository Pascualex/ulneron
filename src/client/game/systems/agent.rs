use bevy::{prelude::*, utils::HashMap};
use kiddo::distance::squared_euclidean;

use crate::{
    client::game::{
        components::{Agent, Position, Size, Stats, Velocity},
        resources::{SpacePartitioner, Ticks},
        utils::orca,
    },
    TICK_STEP,
};

pub fn agent(
    ticks: Res<Ticks>,
    mut query: Query<(Entity, &Position, &Size, &mut Velocity, &Stats, &Agent)>,
    space_partitioner: Res<SpacePartitioner>,
) {
    if ticks.current.is_none() {
        return;
    }
    let mut new_velocities = HashMap::new();
    for (entity, position, size, velocity, stats, agent) in query.iter() {
        let nearest = space_partitioner
            .agents
            .nearest(position.val.as_ref(), 9, &squared_euclidean)
            .unwrap();
        let neighbors = nearest
            .into_iter()
            .filter(|(_, e)| **e != entity)
            .map(|(_, e)| query.get(*e).unwrap())
            .map(|(_, p, s, v, _, _)| (p.val, s.radius, v.val))
            .collect();
        let new_velocity = orca(
            (position.val, size.radius, velocity.val),
            agent.preferred_velocity,
            stats.speed,
            neighbors,
            TICK_STEP,
        );
        new_velocities.insert(entity, new_velocity);
    }
    for (entity, new_velocity) in new_velocities {
        let mut velocity = query.get_mut(entity).unwrap().3;
        velocity.val = new_velocity;
    }
}
