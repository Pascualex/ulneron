use bevy::prelude::*;

use crate::{
    client::game::components::{Position, Velocity},
    client::graphics::resources::TickDelta,
};

pub fn movement(
    mut query: Query<(&mut Transform, &Position, Option<&Velocity>)>,
    tick_delta: Res<TickDelta>,
) {
    for (mut transform, position, velocity) in query.iter_mut() {
        let position = match velocity {
            Some(velocity) => position.val + velocity.val * tick_delta.delta.as_secs_f32(),
            None => position.val,
        };
        transform.translation = Vec3::new(position.y, 0.5, position.x);
    }
}
