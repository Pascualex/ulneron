use bevy::prelude::*;

use crate::{client::components::Velocity, protocol::events::DownstreamEvent, TIME_STEP};

pub fn movement(
    mut downstream_reader: EventReader<DownstreamEvent>,
    mut query: Query<&mut Transform, With<Velocity>>,
) {
    for downstream in downstream_reader.iter() {
        for mut transform in query.iter_mut() {
            let velocity_3d = Vec3::new(downstream.direction.y, 0.0, downstream.direction.x) * 5.0;
            transform.translation += velocity_3d * TIME_STEP;
        }
    }
}
