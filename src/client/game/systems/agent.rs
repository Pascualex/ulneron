use bevy::{prelude::*, utils::HashMap};
use kiddo::distance::squared_euclidean;

use crate::client::game::{
    components::{Agent, Position, Velocity},
    resources::{SpacePartitioner, Ticks},
    utils::orca,
};

pub fn agent(
    ticks: Res<Ticks>,
    mut query: Query<(Entity, &Position, &mut Velocity, &Agent)>,
    space_partitioner: Res<SpacePartitioner>,
) {
    if ticks.current.is_none() {
        return;
    }
    let mut new_velocities = HashMap::new();
    for (entity, position, _, agent) in query.iter() {
        let nearest = space_partitioner
            .tree
            .nearest(position.val.as_ref(), 8, &squared_euclidean)
            .unwrap();
        let neighbors = nearest
            .into_iter()
            .map(|(_, e)| query.get(*e).unwrap())
            .map(|(_, p, v, _)| (p.val, v.val))
            .collect();
        let new_velocity = orca(position.val, agent.preferred_velocity, neighbors);
        new_velocities.insert(entity, new_velocity);
    }
    for (entity, new_velocity) in new_velocities {
        let (_, _, mut velocity, _) = query.get_mut(entity).unwrap();
        velocity.val = new_velocity;
    }
}
