use bevy::prelude::*;
use kiddo::KdTree;

pub struct SpacePartitioner {
    pub players: KdTree<f32, Entity, 2>,
    pub enemies: KdTree<f32, Entity, 2>,
    pub agents: KdTree<f32, Entity, 2>,
}

impl SpacePartitioner {
    pub fn reset(&mut self) {
        self.players = KdTree::new();
        self.enemies = KdTree::new();
        self.agents = KdTree::new();
    }
}

impl Default for SpacePartitioner {
    fn default() -> Self {
        Self {
            players: KdTree::new(),
            enemies: KdTree::new(),
            agents: KdTree::new(),
        }
    }
}
