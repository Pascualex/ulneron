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
        let color = match *player_count % 5 {
            0 => Color::rgb_u8(230, 126, 34),
            1 => Color::rgb_u8(52, 152, 219),
            2 => Color::rgb_u8(155, 89, 182),
            3 => Color::rgb_u8(52, 73, 94),
            _ => Color::rgb_u8(241, 196, 15),
        };
        *player_count += 1;
        commands.entity(entity).insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.25,
                depth: 0.5,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: color,
                metallic: 0.1,
                perceptual_roughness: 0.7,
                reflectance: 0.3,
                ..default()
            }),
            transform: Transform::default(),
            ..default()
        });
    }
}
