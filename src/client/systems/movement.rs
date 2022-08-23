use bevy::prelude::*;

use crate::{
    client::{
        components::Player,
        resources::{PlayerIds, TickBuffer},
    },
    TIME_STEP,
};

pub fn movement(
    tick_buffer: Res<TickBuffer>,
    player_ids: Res<PlayerIds>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let tick = match tick_buffer.ticks.first() {
        Some(tick) => tick,
        None => return,
    };

    for (id, action) in tick.iter() {
        let entity = *player_ids.map.get(id).unwrap();
        let mut transform = query.get_mut(entity).unwrap();
        let velocity_3d = Vec3::new(action.direction.y, 0.0, action.direction.x) * 5.0;
        transform.translation += velocity_3d * TIME_STEP;
    }
}
