use bevy::{prelude::*, utils::HashMap};
use kiddo::distance::squared_euclidean;

use crate::{
    client::game::{
        components::{Agent, Position, Size, Stats, Velocity},
        resources::{SpacePartitioner, Ticks},
        utils::{orca, OrcaAgent},
    },
    TICK_STEP,
};

pub fn agent(
    ticks: Res<Ticks>,
    mut query: Query<(Entity, &Position, &mut Velocity, &Size, &Stats, &Agent)>,
    space_partitioner: Res<SpacePartitioner>,
) {
    if ticks.current.is_none() {
        return;
    }
    let mut new_velocities = HashMap::new();
    for (entity, position, velocity, size, stats, agent) in query.iter() {
        let nearest = space_partitioner
            .agents
            .nearest(position.val.as_ref(), 9, &squared_euclidean)
            .unwrap();
        let neighbors: Vec<_> = nearest
            .into_iter()
            .filter(|(_, e)| **e != entity)
            .map(|(_, e)| query.get(*e).unwrap())
            .map(|(_, p, v, s, _, _)| OrcaAgent::new(p.val, v.val, s.radius))
            .collect();
        let new_velocity = orca(
            OrcaAgent::new(position.val, velocity.val, size.radius),
            &neighbors,
            agent.preferred_velocity,
            stats.speed,
            TICK_STEP,
        );
        new_velocities.insert(entity, new_velocity);
    }
    for (entity, new_velocity) in new_velocities {
        let mut velocity = query.get_mut(entity).unwrap().2;
        velocity.val = new_velocity;
    }
}
