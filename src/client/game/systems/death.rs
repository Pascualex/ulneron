use bevy::prelude::*;

use crate::client::game::{components::Health, resources::Ticks};

pub fn death(
    ticks: Res<Ticks>,
    query: Query<(Entity, &Health), Changed<Health>>,
    mut commands: Commands,
) {
    if ticks.current.is_none() {
        return;
    }

    for (entity, health) in query.iter() {
        if health.dead() {
            commands.entity(entity).despawn();
        }
    }
}
