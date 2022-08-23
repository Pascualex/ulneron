use bevy::prelude::*;
use uuid::Uuid;

use crate::protocol::{data::Action, events::UpstreamEvent};

pub fn setup(
    mut commands: Commands,
    id: Res<Uuid>,
    mut upstream_writer: EventWriter<UpstreamEvent>,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-5.0, 15.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ZERO)
            .looking_at(Vec3::new(-1.0, -2.0, 1.0), Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 32_000.0,
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ZERO)
            .looking_at(Vec3::new(1.0, -2.0, -1.0), Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 10_000.0,
            ..default()
        },
        ..default()
    });
    upstream_writer.send(UpstreamEvent::new(*id, Action::new()));
}
