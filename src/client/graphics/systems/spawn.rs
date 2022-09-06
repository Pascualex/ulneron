use bevy::prelude::*;

use crate::client::{
    controller::resources::ControllerInfo,
    game::components::{Player, Position},
    graphics::components::LocalPlayer,
    lobby::resources::PlayersInfo,
};

pub fn spawn(
    query: Query<(Entity, &Position, Option<&Player>), Added<Position>>,
    players_info: Res<PlayersInfo>,
    controller_info: Res<ControllerInfo>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, position, player) in query.iter() {
        let color = match player {
            Some(player) => match player.id % 5 {
                0 => Color::rgb_u8(230, 126, 34),
                1 => Color::rgb_u8(52, 152, 219),
                2 => Color::rgb_u8(155, 89, 182),
                3 => Color::rgb_u8(52, 73, 94),
                _ => Color::rgb_u8(241, 196, 15),
            },
            None => Color::BLACK,
        };
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
            transform: Transform::from_xyz(position.value.y, 0.5, position.value.x),
            ..default()
        });
        if let Some(player) = player {
            let uuid = players_info.uuids[player.id];
            if uuid == controller_info.uuid {
                commands.entity(entity).insert(LocalPlayer);
            }
        }
    }
}
