use bevy::prelude::*;

use crate::{
    client::{components::Player, resources::PlayerIds},
    protocol::events::DownstreamEvent,
    TIME_STEP,
};

pub fn movement(
    mut downstream_reader: EventReader<DownstreamEvent>,
    player_ids: Res<PlayerIds>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for downstream in downstream_reader.iter() {
        for (id, direction) in downstream.directions.iter() {
            let entity = *player_ids.map.get(id).unwrap();
            let mut transform = query.get_mut(entity).unwrap();
            let velocity_3d = Vec3::new(direction.y, 0.0, direction.x) * 5.0;
            transform.translation += velocity_3d * TIME_STEP;
        }
    }
}
