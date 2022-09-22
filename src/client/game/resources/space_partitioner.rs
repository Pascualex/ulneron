use bevy::prelude::*;
use kiddo::KdTree;

pub struct SpacePartitioner {
    pub tree: KdTree<f32, Entity, 2>,
}

impl Default for SpacePartitioner {
    fn default() -> Self {
        Self {
            tree: KdTree::new(),
        }
    }
}
