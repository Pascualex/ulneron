use bevy::prelude::*;

use crate::{
    client::{components::Velocity, resources::EntitiesIds},
    events::downstream::SpawnEvent,
};

pub fn spawn_sync(
    mut spawn_reader: ResMut<Events<SpawnEvent>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entities_ids: ResMut<EntitiesIds>,
) {
    for spawn in spawn_reader.drain() {
        if entities_ids.map.contains_key(&spawn.id) {
            continue;
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
        entities_ids.map.insert(spawn.id, entity);
    }
}
