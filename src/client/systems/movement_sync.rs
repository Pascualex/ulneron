use bevy::prelude::*;

use crate::{
    client::{components::Velocity, resources::EntitiesIds},
    events::downstream::MovementEvent,
};

pub fn movement_sync(
    mut movement_reader: EventReader<MovementEvent>,
    entities_ids: Res<EntitiesIds>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    for movement in movement_reader.iter() {
        let entity = match entities_ids.map.get(&movement.id) {
            Some(entity) => *entity,
            None => continue,
        };
        let (mut transform, mut velocity) = match query.get_mut(entity) {
            Ok(result) => result,
            Err(_) => continue,
        };
        transform.translation = Vec3::new(movement.position.y, 0.5, movement.position.x);
        velocity.value = movement.velocity;
    }
}
