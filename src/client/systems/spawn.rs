use bevy::prelude::*;

use crate::client::{
    components::Player,
    resources::{PlayerIds, TickBuffer},
};

pub fn spawn(
    tick_buffer: Res<TickBuffer>,
    mut player_ids: ResMut<PlayerIds>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tick = match tick_buffer.ticks.first() {
        Some(tick) => tick,
        None => return,
    };

    for id in tick.keys() {
        if player_ids.map.contains_key(id) {
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
            .insert(Player)
            .id();
        player_ids.map.try_insert(*id, entity).unwrap();
    }
}
