use bevy::prelude::*;

use crate::client::components::Player;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
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
        .insert(Player);
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
}
