use bevy::prelude::*;

use crate::client::components::Velocity;

pub fn spawn(
    mut entity_spawned: ResMut<Option<Entity>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if entity_spawned.is_some() {
        return;
    }

    let entity = commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.25,
                depth: 0.5,
                ..default()
            })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Velocity::from_xy(0.0, 0.0))
        .id();
    let _ = entity_spawned.insert(entity);
}
