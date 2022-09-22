use bevy::prelude::*;
use kiddo::KdTree;

use crate::client::game::{components::Position, resources::SpacePartitioner};

pub fn space_partitioner(
    mut query: Query<(Entity, &Position)>,
    mut space_partitioner: ResMut<SpacePartitioner>,
) {
    space_partitioner.tree = KdTree::new();
    for (entity, position) in query.iter_mut() {
        space_partitioner
            .tree
            .add(position.val.as_ref(), entity)
            .unwrap();
    }
}
