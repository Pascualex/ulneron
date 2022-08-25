use bevy::prelude::*;

use crate::client::components::Velocity;

pub fn movement_view(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        let delta = velocity.value * time.delta().as_secs_f32();
        transform.translation += Vec3::new(delta.y, 0.0, delta.x);
    }
}
