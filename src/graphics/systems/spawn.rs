use bevy::prelude::*;

use crate::client::components::Position;

pub fn spawn(
    query: Query<Entity, Added<Position>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.25,
                depth: 0.5,
                ..default()
            })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::default(),
            ..default()
        });
    }
}
