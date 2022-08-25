use bevy::prelude::*;

use crate::client::components::{Player, Position};

pub fn spawn(
    query: Query<(Entity, &Player), Added<Position>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, player) in query.iter() {
        let color = match player.id % 10 {
            0 => Color::WHITE,
            1 => Color::BLACK,
            2 => Color::BLUE,
            3 => Color::RED,
            4 => Color::GREEN,
            5 => Color::YELLOW,
            6 => Color::PINK,
            7 => Color::ORANGE,
            8 => Color::TEAL,
            9 => Color::PURPLE,
            _ => panic!(),
        };
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
