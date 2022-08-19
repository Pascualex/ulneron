use bevy::prelude::*;

use crate::client::components::Velocity;

pub fn movement(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        let velocity_3d = Vec3::new(velocity.value.y, 0.0, velocity.value.x);
        transform.translation += velocity_3d * time.delta().as_secs_f32();
    }
}
