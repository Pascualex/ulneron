use bevy::{prelude::*, utils::HashMap};
use kiddo::distance::squared_euclidean;

use crate::{
    client::game::{
        components::{Agent, Position, Size, Velocity},
        resources::{SpacePartitioner, Ticks},
        utils::orca,
    },
    TICK_STEP,
};

pub fn agent(
    ticks: Res<Ticks>,
    mut query: Query<(Entity, &Position, &Size, &mut Velocity, &Agent)>,
    space_partitioner: Res<SpacePartitioner>,
) {
    if ticks.current.is_none() {
        return;
    }
    let mut new_velocities = HashMap::new();
    for (entity, position, size, _, agent) in query.iter() {
        let nearest = space_partitioner
            .agents
            .nearest(position.val.as_ref(), 8, &squared_euclidean)
            .unwrap();
        let neighbors = nearest
            .into_iter()
            .map(|(_, e)| query.get(*e).unwrap())
            .map(|(_, p, s, v, _)| (p.val, s.radius, v.val))
            .collect();
        let new_velocity = orca(
            position.val,
            size.radius,
            agent.preferred_velocity,
            neighbors,
            TICK_STEP,
        );
        new_velocities.insert(entity, new_velocity);
    }
    for (entity, new_velocity) in new_velocities {
        let (_, _, _, mut velocity, _) = query.get_mut(entity).unwrap();
        velocity.val = new_velocity;
    }
}
