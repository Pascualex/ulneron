use bevy::prelude::*;

pub fn orca(
    position: Vec2,
    size: f32,
    preferred_velocity: Vec2,
    neighbors: Vec<(Vec2, f32, Vec2)>,
) -> Vec2 {
    for (neighbor_position, neighbor_size, neighbor_velocity) in neighbors.iter() {
        // construct VO(current|neighbor)
        // construct ORCA(current|neighbor)
    }
    // construct ORCA(current) from the intersection of ORCA(current|neighbors)
    // compute new velocity for current
    preferred_velocity
}
