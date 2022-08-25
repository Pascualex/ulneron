use bevy::prelude::*;

use crate::client::components::Position;

pub fn spawn(
    query: Query<Entity, Added<Position>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_count: Local<u32>,
) {
    for entity in query.iter() {
        let color = match *player_count % 10 {
            0 => Color::WHITE,
            1 => Color::BLUE,
            2 => Color::RED,
            3 => Color::GREEN,
            4 => Color::YELLOW,
            5 => Color::PINK,
            6 => Color::ORANGE,
            7 => Color::TEAL,
            8 => Color::PURPLE,
            _ => Color::BLACK,
        };
        *player_count += 1;
        commands.entity(entity).insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.25,
                depth: 0.5,
                ..default()
            })),
            material: materials.add(color.into()),
            transform: Transform::default(),
            ..default()
        });
    }
}
